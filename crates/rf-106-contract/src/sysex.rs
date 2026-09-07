//! Roland JUNO-106 tone data and System Exclusive file codec.
//!
//! The original MIDI implementation transports one complete tone as sixteen
//! seven-bit potentiometer values followed by two packed switch bytes.  This
//! module keeps those eighteen bytes as the portable source of truth.  Engine
//! parameters are only a view of them, so a file can round-trip without losing
//! hardware values that RF-106 has to clamp while sounding them.

use super::{
    FactoryPreset, NATIVE_CHORUS_I, NATIVE_CHORUS_II, NATIVE_DCO_LFO, NATIVE_DCO_NOISE_LEVEL,
    NATIVE_DCO_PULSE, NATIVE_DCO_PWM, NATIVE_DCO_RANGE, NATIVE_DCO_SAW, NATIVE_DCO_SUB_LEVEL,
    NATIVE_ENV_ATTACK, NATIVE_ENV_DECAY, NATIVE_ENV_RELEASE, NATIVE_ENV_SUSTAIN, NATIVE_HPF,
    NATIVE_LFO_DELAY, NATIVE_LFO_RATE, NATIVE_PARAMETER_COUNT, NATIVE_PWM_MODE, NATIVE_VCA_LEVEL,
    NATIVE_VCA_MODE, NATIVE_VCF_CUTOFF, NATIVE_VCF_ENV, NATIVE_VCF_ENV_POLARITY,
    NATIVE_VCF_KEYBOARD, NATIVE_VCF_LFO, NATIVE_VCF_RESONANCE, native_parameter_value_is_valid,
};

pub const TONE_BYTES: usize = 18;
pub const APR_BYTES: usize = 24;
pub const ROLAND_ID: u8 = 0x41;
pub const APR_PROGRAM: u8 = 0x30;
pub const APR_MANUAL: u8 = 0x31;
pub const IPR_PARAMETER: u8 = 0x32;
pub const PWM_HARDWARE_MAX: u8 = 105;

const CONTINUOUS_NATIVE: [usize; 16] = [
    NATIVE_LFO_RATE,
    NATIVE_LFO_DELAY,
    NATIVE_DCO_LFO,
    NATIVE_DCO_PWM,
    NATIVE_DCO_NOISE_LEVEL,
    NATIVE_VCF_CUTOFF,
    NATIVE_VCF_RESONANCE,
    NATIVE_VCF_ENV,
    NATIVE_VCF_LFO,
    NATIVE_VCF_KEYBOARD,
    NATIVE_VCA_LEVEL,
    NATIVE_ENV_ATTACK,
    NATIVE_ENV_DECAY,
    NATIVE_ENV_SUSTAIN,
    NATIVE_ENV_RELEASE,
    NATIVE_DCO_SUB_LEVEL,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Tone {
    bytes: [u8; TONE_BYTES],
}

impl Tone {
    pub const fn new(bytes: [u8; TONE_BYTES]) -> Option<Self> {
        let mut index = 0;
        while index < TONE_BYTES {
            if bytes[index] > 0x7f {
                return None;
            }
            index += 1;
        }
        Some(Self { bytes })
    }

    pub const fn bytes(&self) -> &[u8; TONE_BYTES] {
        &self.bytes
    }

    /// Reconstruct the documented tone bytes from the imported factory table.
    /// Its continuous values retain the original raw-byte/127 representation.
    pub fn from_factory(preset: &FactoryPreset) -> Self {
        let mut bytes = [0_u8; TONE_BYTES];
        for (slot, native) in CONTINUOUS_NATIVE.iter().copied().enumerate() {
            bytes[slot] = unit_to_byte(preset.values[native], 127);
        }
        bytes[16] = encode_switch_one(&preset.values);
        bytes[17] = encode_switch_two(&preset.values);
        Self { bytes }
    }

    /// Capture the current RF-106 tone. Performance and host controls are not
    /// represented, exactly like the original eighteen-byte report.
    pub fn from_native(values: &[f64; NATIVE_PARAMETER_COUNT]) -> Option<Self> {
        if values
            .iter()
            .enumerate()
            .any(|(index, value)| !native_parameter_value_is_valid(index as u32, *value))
        {
            return None;
        }
        let mut bytes = [0_u8; TONE_BYTES];
        for (slot, native) in CONTINUOUS_NATIVE.iter().copied().enumerate() {
            let maximum = if slot == 3 { PWM_HARDWARE_MAX } else { 127 };
            bytes[slot] = unit_to_byte(values[native], maximum);
        }
        bytes[16] = encode_switch_one(values);
        bytes[17] = encode_switch_two(values);
        Some(Self { bytes })
    }

    /// Apply only patch-memory controls. Tuning, bender, MIDI, allocation,
    /// portamento, key transpose, power and output controls remain untouched.
    pub fn apply_to_native(&self, values: &mut [f64; NATIVE_PARAMETER_COUNT]) {
        for (slot, native) in CONTINUOUS_NATIVE.iter().copied().enumerate() {
            let denominator = if slot == 3 {
                f64::from(PWM_HARDWARE_MAX)
            } else {
                127.0
            };
            values[native] = (f64::from(self.bytes[slot]) / denominator).min(1.0);
        }

        let switches = self.bytes[16];
        values[NATIVE_DCO_RANGE] = if switches & 0x04 != 0 {
            2.0
        } else if switches & 0x02 != 0 {
            1.0
        } else {
            0.0
        };
        values[NATIVE_DCO_PULSE] = bit_value(switches, 0x08);
        values[NATIVE_DCO_SAW] = bit_value(switches, 0x10);
        let chorus_on = switches & 0x20 == 0;
        let chorus_i = switches & 0x40 != 0;
        values[NATIVE_CHORUS_I] = u8::from(chorus_on && chorus_i) as f64;
        values[NATIVE_CHORUS_II] = u8::from(chorus_on && !chorus_i) as f64;

        let switches = self.bytes[17];
        values[NATIVE_PWM_MODE] = bit_value(switches, 0x01);
        values[NATIVE_VCF_ENV_POLARITY] = bit_value(switches, 0x02);
        values[NATIVE_VCA_MODE] = bit_value(switches, 0x04);
        values[NATIVE_HPF] = f64::from(3 - ((switches >> 3) & 0x03));
    }

    pub fn encode_apr(self, function: u8, channel: u8, patch: u8) -> Option<[u8; APR_BYTES]> {
        if !matches!(function, APR_PROGRAM | APR_MANUAL) || channel > 15 || patch > 127 {
            return None;
        }
        let mut message = [0_u8; APR_BYTES];
        message[0] = 0xf0;
        message[1] = ROLAND_ID;
        message[2] = function;
        message[3] = channel;
        message[4] = patch;
        message[5..23].copy_from_slice(&self.bytes);
        message[23] = 0xf7;
        Some(message)
    }

    pub fn decode_apr(message: &[u8]) -> Option<Apr> {
        if message.len() != APR_BYTES
            || message[0] != 0xf0
            || message[1] != ROLAND_ID
            || !matches!(message[2], APR_PROGRAM | APR_MANUAL)
            || message[3] > 15
            || message[4] > 127
            || message[23] != 0xf7
        {
            return None;
        }
        let bytes: [u8; TONE_BYTES] = message[5..23].try_into().ok()?;
        Some(Apr {
            function: message[2],
            channel: message[3],
            patch: message[4],
            tone: Self::new(bytes)?,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Apr {
    pub function: u8,
    pub channel: u8,
    pub patch: u8,
    pub tone: Tone,
}

fn bit_value(byte: u8, mask: u8) -> f64 {
    u8::from(byte & mask != 0) as f64
}

fn unit_to_byte(value: f64, maximum: u8) -> u8 {
    libm::round(value.clamp(0.0, 1.0) * f64::from(maximum)) as u8
}

fn encode_switch_one(values: &[f64; NATIVE_PARAMETER_COUNT]) -> u8 {
    let mut byte = match libm::round(values[NATIVE_DCO_RANGE]) as u8 {
        2 => 0x04,
        1 => 0x02,
        _ => 0x01,
    };
    if values[NATIVE_DCO_PULSE] >= 0.5 {
        byte |= 0x08;
    }
    if values[NATIVE_DCO_SAW] >= 0.5 {
        byte |= 0x10;
    }
    let chorus_i = values[NATIVE_CHORUS_I] >= 0.5;
    let chorus_ii = values[NATIVE_CHORUS_II] >= 0.5;
    if !chorus_i && !chorus_ii {
        byte |= 0x20;
    } else if chorus_i {
        byte |= 0x40;
    }
    byte
}

fn encode_switch_two(values: &[f64; NATIVE_PARAMETER_COUNT]) -> u8 {
    let mut byte = 0_u8;
    if values[NATIVE_PWM_MODE] >= 0.5 {
        byte |= 0x01;
    }
    if values[NATIVE_VCF_ENV_POLARITY] >= 0.5 {
        byte |= 0x02;
    }
    if values[NATIVE_VCA_MODE] >= 0.5 {
        byte |= 0x04;
    }
    let hpf = libm::round(values[NATIVE_HPF]).clamp(0.0, 3.0) as u8;
    byte | ((3 - hpf) << 3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FACTORY_PRESET_COUNT, factory_preset};
    use sha2::{Digest, Sha256};

    #[test]
    fn original_factory_tones_round_trip_through_apr() {
        for index in 0..FACTORY_PRESET_COUNT as u32 {
            let tone = Tone::from_factory(factory_preset(index).unwrap());
            let message = tone.encode_apr(APR_PROGRAM, 15, index as u8).unwrap();
            let decoded = Tone::decode_apr(&message).unwrap();
            assert_eq!(decoded.patch, index as u8);
            assert_eq!(decoded.channel, 15);
            assert_eq!(decoded.tone, tone);
        }
    }

    #[test]
    fn first_factory_patch_is_the_original_eighteen_bytes() {
        let tone = Tone::from_factory(factory_preset(0).unwrap());
        assert_eq!(
            tone.bytes(),
            &[
                0x14, 0x31, 0x00, 0x66, 0x00, 0x23, 0x0d, 0x3a, 0x00, 0x56, 0x6c, 0x03, 0x31, 0x2d,
                0x20, 0x00, 0x51, 0x11
            ]
        );
    }

    #[test]
    fn complete_factory_tone_data_matches_the_original_bank() {
        let mut digest = Sha256::new();
        for index in 0..FACTORY_PRESET_COUNT as u32 {
            digest.update(Tone::from_factory(factory_preset(index).unwrap()).bytes());
        }
        assert_eq!(
            digest.finalize().as_slice(),
            &[
                0x39, 0x4a, 0xe8, 0x74, 0xda, 0x33, 0xaa, 0x63, 0xfa, 0x48, 0x33, 0x93, 0x2f, 0xbf,
                0x41, 0x55, 0x46, 0xd2, 0xad, 0x66, 0xb1, 0xb6, 0xb9, 0xa3, 0x63, 0x15, 0x60, 0x17,
                0x99, 0xee, 0xec, 0x21,
            ]
        );
    }

    #[test]
    fn engine_view_scales_the_hardware_limited_pwm_and_preserves_globals() {
        let tone = Tone::new([
            127, 64, 32, 105, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 0x5a, 0x1f,
        ])
        .unwrap();
        let mut values = factory_preset(0).unwrap().values;
        let bender = values[0];
        tone.apply_to_native(&mut values);
        assert_eq!(values[NATIVE_DCO_PWM], 1.0);
        assert_eq!(values[NATIVE_DCO_SUB_LEVEL], 12.0 / 127.0);
        assert_eq!(values[0], bender, "bender is not patch memory");
        assert_eq!(Tone::from_native(&values).unwrap(), tone);
    }

    #[test]
    fn malformed_or_non_seven_bit_messages_are_refused() {
        let tone = Tone::new([0; TONE_BYTES]).unwrap();
        let mut message = tone.encode_apr(APR_MANUAL, 0, 0).unwrap();
        message[8] = 0x80;
        assert!(Tone::decode_apr(&message).is_none());
        message[8] = 0;
        message[2] = IPR_PARAMETER;
        assert!(Tone::decode_apr(&message).is_none());
    }
}
