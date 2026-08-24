#![no_std]

pub const PLUGIN_ID: &str = "org.rackforge.rf-106";
pub const PLUGIN_VERSION: &str = "0.2.7";
pub const STATE_SCHEMA_VERSION: u32 = 1;
pub const DEFAULT_FACTORY_PROGRAM: u32 = 0;
pub const NATIVE_PARAMETER_COUNT: usize = 56;
/// RackForge parameter indices are append-only. The compact surface
/// occupies 0..=13; the complete RF-106 audio panel starts at 14 so existing
/// sessions and automation lanes keep their identity.
pub const PUBLIC_PARAMETER_COUNT: usize = 39;
pub const PUBLIC_LFO_TRIGGER_INDEX: u32 = 13;
pub const PUBLIC_CHORUS_MODE_INDEX: u32 = 31;
pub const PUBLIC_MIDI_CHANNEL_INDEX: u32 = 33;
pub const PUBLIC_MIDI_FUNCTION_INDEX: u32 = 34;
/// Physical Bender Board SW1. It is append-only and deliberately synthetic:
/// the switch gates the analog portamento voltage before Voice ADC AN1, so it
/// has no native patch-memory slot.
pub const PUBLIC_PORTAMENTO_SWITCH_INDEX: u32 = 35;
/// Physical dual-gang Bender Board VR1. It lives after the stereo chorus and
/// has no patch-memory/native CPU slot.
pub const PUBLIC_PHYSICAL_VOLUME_INDEX: u32 = 36;
/// Momentary front-panel Key Transpose button. While held, the next playable
/// keyboard note selects the firmware-derived -12..+12 transpose offset.
pub const PUBLIC_KEY_TRANSPOSE_TRIGGER_INDEX: u32 = 37;
/// Spring-centred horizontal position of the physical bender lever. This is a
/// performance input, not patch memory; pushing the same lever forward uses
/// the existing momentary `PUBLIC_LFO_TRIGGER_INDEX` path.
pub const PUBLIC_BENDER_POSITION_INDEX: u32 = 38;
pub const FACTORY_PRESET_COUNT: usize = 128;

#[derive(Clone, Copy, Debug)]
pub struct FactoryPreset {
    pub name: &'static str,
    pub values: [f64; NATIVE_PARAMETER_COUNT],
}

mod factory_presets;
pub use factory_presets::FACTORY_PRESETS;

pub fn factory_preset(index: u32) -> Option<&'static FactoryPreset> {
    FACTORY_PRESETS.get(index as usize)
}

pub fn parse_preset_id(id: &str) -> Option<u32> {
    let (bank, number) = id.rsplit_once('.')?;
    let number = parse_decimal(number)?;
    (bank == "factory.rf106" && number < FACTORY_PRESET_COUNT as u32).then_some(number)
}

pub const fn public_to_native_parameter(public_index: u32) -> Option<u32> {
    match public_index {
        0 => Some(44),
        1 => Some(10),
        2 => Some(11),
        3 => Some(16),
        4 => Some(17),
        5 => Some(18),
        6 => Some(19),
        7 => Some(0),
        8 => Some(1),
        9 => Some(42),
        10 => Some(37),
        11 => Some(40),
        12 => Some(39),
        14 => Some(3),
        15 => Some(4),
        16 => Some(29),
        17 => Some(5),
        18 => Some(6),
        19 => Some(33),
        20 => Some(23),
        21 => Some(24),
        22 => Some(7),
        23 => Some(8),
        24 => Some(9),
        25 => Some(12),
        26 => Some(34),
        27 => Some(13),
        28 => Some(14),
        29 => Some(35),
        30 => Some(15),
        32 => Some(38),
        _ => None,
    }
}

pub fn public_parameter_value_is_valid(public_index: u32, value: f64) -> bool {
    if public_index == PUBLIC_LFO_TRIGGER_INDEX {
        return value == 0.0 || value == 1.0;
    }
    if public_index == PUBLIC_CHORUS_MODE_INDEX {
        // 0/1/2 are the three firmware states. Hidden value 3 remains valid
        // solely so older RF-106 automation can be canonicalized to mode I.
        return value == value as i64 as f64 && (0.0..=3.0).contains(&value);
    }
    if public_index == PUBLIC_MIDI_CHANNEL_INDEX {
        return value == value as i64 as f64 && (1.0..=16.0).contains(&value);
    }
    if public_index == PUBLIC_MIDI_FUNCTION_INDEX {
        return value == value as i64 as f64 && (1.0..=3.0).contains(&value);
    }
    if public_index == PUBLIC_PORTAMENTO_SWITCH_INDEX {
        return value == 0.0 || value == 1.0;
    }
    if public_index == PUBLIC_PHYSICAL_VOLUME_INDEX {
        return value.is_finite() && (0.0..=1.0).contains(&value);
    }
    if public_index == PUBLIC_KEY_TRANSPOSE_TRIGGER_INDEX {
        return value == 0.0 || value == 1.0;
    }
    if public_index == PUBLIC_BENDER_POSITION_INDEX {
        return value.is_finite() && (-1.0..=1.0).contains(&value);
    }
    let Some(native_index) = public_to_native_parameter(public_index) else {
        return false;
    };
    native_parameter_value_is_valid(native_index, value)
}

pub fn native_parameter_value_is_valid(index: u32, value: f64) -> bool {
    if !value.is_finite() {
        return false;
    }
    let integral = value == value as i64 as f64;
    match index {
        0..=8 | 10..=19 | 40 | 42..=44 => (0.0..=1.0).contains(&value),
        9 => integral && (0.0..=3.0).contains(&value),
        20..=28 | 32 | 34..=35 | 38 | 47..=52 | 55 => integral && (0.0..=1.0).contains(&value),
        29..=31 | 33 | 39 => integral && (0.0..=2.0).contains(&value),
        36..=37 => (-1.0..=1.0).contains(&value),
        // A_5 Key Transpose state is exactly one octave down/up. The retained
        // native slot is now constrained to the executed hardware domain.
        41 => integral && (-12.0..=12.0).contains(&value),
        45 => matches!(value as i32, 6 | 8 | 10) && integral,
        46 => matches!(value as i32, 1 | 2 | 4) && integral,
        53 => integral && (0.0..=8.0).contains(&value),
        54 => integral && (0.0..=12.0).contains(&value),
        _ => false,
    }
}

fn parse_decimal(text: &str) -> Option<u32> {
    if text.is_empty() {
        return None;
    }
    let mut value = 0_u32;
    for byte in text.bytes() {
        if !byte.is_ascii_digit() {
            return None;
        }
        value = value.checked_mul(10)?.checked_add(u32::from(byte - b'0'))?;
    }
    Some(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_factory_contract_is_complete() {
        assert_eq!(FACTORY_PRESETS.len(), 128);
        assert_eq!(FACTORY_PRESETS[0].name, "A11 Brass");
        assert_eq!(FACTORY_PRESETS[127].values.len(), NATIVE_PARAMETER_COUNT);
        assert!(
            FACTORY_PRESETS
                .iter()
                .all(|preset| preset.values[27] == 0.0 || preset.values[28] == 0.0),
            "factory programs must use only Off, I or II"
        );
    }

    #[test]
    fn preset_ids_are_strict_and_stable() {
        assert_eq!(parse_preset_id("factory.rf106.000"), Some(0));
        assert_eq!(parse_preset_id("factory.rf106.127"), Some(127));
        assert_eq!(parse_preset_id("factory.rf106.128"), None);
        assert_eq!(parse_preset_id("factory.rf106.-1"), None);
    }

    #[test]
    fn public_parameter_mapping_matches_native_layout() {
        assert_eq!(public_to_native_parameter(0), Some(44));
        assert_eq!(public_to_native_parameter(1), Some(10));
        assert_eq!(public_to_native_parameter(6), Some(19));
        assert_eq!(public_to_native_parameter(7), Some(0));
        assert_eq!(public_to_native_parameter(10), Some(37));
        assert_eq!(public_to_native_parameter(11), Some(40));
        assert_eq!(public_to_native_parameter(12), Some(39));
        assert_eq!(public_to_native_parameter(PUBLIC_LFO_TRIGGER_INDEX), None);
        assert_eq!(public_to_native_parameter(14), Some(3));
        assert_eq!(public_to_native_parameter(16), Some(29));
        assert_eq!(public_to_native_parameter(24), Some(9));
        assert_eq!(public_to_native_parameter(30), Some(15));
        assert_eq!(public_to_native_parameter(PUBLIC_CHORUS_MODE_INDEX), None);
        assert_eq!(public_to_native_parameter(32), Some(38));
        assert_eq!(public_to_native_parameter(PUBLIC_MIDI_CHANNEL_INDEX), None);
        assert_eq!(public_to_native_parameter(PUBLIC_MIDI_FUNCTION_INDEX), None);
        assert_eq!(
            public_to_native_parameter(PUBLIC_PORTAMENTO_SWITCH_INDEX),
            None
        );
        assert_eq!(
            public_to_native_parameter(PUBLIC_PHYSICAL_VOLUME_INDEX),
            None
        );
        assert!(public_parameter_value_is_valid(
            PUBLIC_LFO_TRIGGER_INDEX,
            0.0
        ));
        assert!(public_parameter_value_is_valid(
            PUBLIC_LFO_TRIGGER_INDEX,
            1.0
        ));
        assert!(!public_parameter_value_is_valid(
            PUBLIC_LFO_TRIGGER_INDEX,
            0.5
        ));
        for mode in 0..=2 {
            assert!(public_parameter_value_is_valid(
                PUBLIC_CHORUS_MODE_INDEX,
                f64::from(mode)
            ));
        }
        assert!(public_parameter_value_is_valid(
            PUBLIC_CHORUS_MODE_INDEX,
            3.0
        ));
        assert!(!public_parameter_value_is_valid(
            PUBLIC_CHORUS_MODE_INDEX,
            1.5
        ));
        assert!(public_parameter_value_is_valid(
            PUBLIC_MIDI_CHANNEL_INDEX,
            16.0
        ));
        assert!(!public_parameter_value_is_valid(
            PUBLIC_MIDI_CHANNEL_INDEX,
            0.0
        ));
        assert!(public_parameter_value_is_valid(
            PUBLIC_MIDI_FUNCTION_INDEX,
            3.0
        ));
        assert!(!public_parameter_value_is_valid(
            PUBLIC_MIDI_FUNCTION_INDEX,
            4.0
        ));
        assert!(public_parameter_value_is_valid(
            PUBLIC_PORTAMENTO_SWITCH_INDEX,
            0.0
        ));
        assert!(public_parameter_value_is_valid(
            PUBLIC_PORTAMENTO_SWITCH_INDEX,
            1.0
        ));
        assert!(!public_parameter_value_is_valid(
            PUBLIC_PORTAMENTO_SWITCH_INDEX,
            0.5
        ));
        assert!(public_parameter_value_is_valid(
            PUBLIC_PHYSICAL_VOLUME_INDEX,
            0.0
        ));
        assert!(public_parameter_value_is_valid(
            PUBLIC_PHYSICAL_VOLUME_INDEX,
            0.73
        ));
        assert!(public_parameter_value_is_valid(
            PUBLIC_PHYSICAL_VOLUME_INDEX,
            1.0
        ));
        assert!(!public_parameter_value_is_valid(
            PUBLIC_PHYSICAL_VOLUME_INDEX,
            f64::NAN
        ));
        assert!(!public_parameter_value_is_valid(
            PUBLIC_PHYSICAL_VOLUME_INDEX,
            1.01
        ));
        assert!(public_parameter_value_is_valid(10, -1.0));
        assert!(!public_parameter_value_is_valid(10, -1.01));
        assert!(native_parameter_value_is_valid(41, -12.0));
        assert!(native_parameter_value_is_valid(41, 12.0));
        assert!(!native_parameter_value_is_valid(41, -13.0));
        assert!(!native_parameter_value_is_valid(41, 13.0));
    }
}
