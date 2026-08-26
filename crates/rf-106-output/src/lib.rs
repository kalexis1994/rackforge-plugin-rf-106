#![no_std]

use core::f32::consts::PI;

/// Effective resistance seen by the two switched HPF capacitors:
/// 1 MOhm bias resistance in parallel with the 47 kOhm virtual-ground path.
pub const HPF_EFFECTIVE_RESISTANCE_OHMS: f32 = 44_890.164;
pub const HPF_LOW_CAPACITANCE_FARADS: f32 = 0.015e-6;
pub const HPF_HIGH_CAPACITANCE_FARADS: f32 = 0.0047e-6;

pub const fn rc_cutoff_hz(resistance_ohms: f32, capacitance_farads: f32) -> f32 {
    1.0 / (2.0 * PI * resistance_ohms * capacitance_farads)
}

pub const HPF_LOW_CUTOFF_HZ: f32 =
    rc_cutoff_hz(HPF_EFFECTIVE_RESISTANCE_OHMS, HPF_LOW_CAPACITANCE_FARADS);
pub const HPF_HIGH_CUTOFF_HZ: f32 =
    rc_cutoff_hz(HPF_EFFECTIVE_RESISTANCE_OHMS, HPF_HIGH_CAPACITANCE_FARADS);

/// Module Board IC1a sums the six A1QH80017A outputs through one 33 kOhm
/// resistor per voice and a 3.3 kOhm feedback resistor. The ideal small-signal
/// contribution of each voice is therefore inverted and attenuated by ten.
pub const VOICE_SUM_INPUT_RESISTANCE_OHMS: f32 = 33_000.0;
pub const VOICE_SUM_FEEDBACK_RESISTANCE_OHMS: f32 = 3_300.0;
pub const VOICE_SUM_GAIN_PER_VOICE: f32 =
    -VOICE_SUM_FEEDBACK_RESISTANCE_OHMS / VOICE_SUM_INPUT_RESISTANCE_OHMS;

/// Service adjustment 6-1 sets every A1QH80017A VCA output to a 6 Vp-p sine.
/// The DSP voice convention therefore maps a unit peak to 3 physical volts.
pub const SERVICE_VCA_OUTPUT_VPP: f32 = 6.0;
pub const VOICE_UNIT_PEAK_VOLTS: f32 = SERVICE_VCA_OUTPUT_VPP * 0.5;

/// IC1a is an M5218L on the documented +/-15 V rails. Mitsubishi's
/// contemporary data book specifies at least +/-12 V output swing into loads
/// of 10 kOhm or greater, +/-14 V typical, 2.2 V/us slew and 7 MHz GBW.
pub const M5218_SUPPLY_VOLTS: f32 = 15.0;
pub const M5218_MIN_OUTPUT_PEAK_VOLTS: f32 = 12.0;
pub const M5218_TYP_OUTPUT_PEAK_VOLTS: f32 = 14.0;
pub const M5218_SLEW_RATE_VOLTS_PER_US: f32 = 2.2;
pub const M5218_GAIN_BANDWIDTH_HZ: f32 = 7_000_000.0;
pub const M5218_LINEAR_PEAK_UNITS: f32 = M5218_MIN_OUTPUT_PEAK_VOLTS / VOICE_UNIT_PEAK_VOLTS;
pub const M5218_TYP_PEAK_UNITS: f32 = M5218_TYP_OUTPUT_PEAK_VOLTS / VOICE_UNIT_PEAK_VOLTS;

/// Bender Board VR1 is a dual-gang 10 kOhm B-taper potentiometer between each
/// chorus output and ground. Its two wipers feed the Jack Board.
pub const BENDER_VOLUME_TRACK_RESISTANCE_OHMS: f32 = 10_000.0;
/// Each Jack Board headphone-amplifier input receives the wiper through 1 kOhm.
pub const HEADPHONE_INPUT_RESISTANCE_OHMS: f32 = 1_000.0;
/// R63/R59/R58 (or R62/R61/R60) remain a 33k + 68k + 56k path to ground,
/// independently of the selected H/M/L output-level tap.
pub const OUTPUT_LEVEL_LADDER_RESISTANCE_OHMS: f32 = 33_000.0 + 68_000.0 + 56_000.0;
/// Nominal load seen by either VR1 wiper with no external output/headphone load.
pub const BENDER_VOLUME_LOAD_RESISTANCE_OHMS: f32 =
    1.0 / (1.0 / HEADPHONE_INPUT_RESISTANCE_OHMS + 1.0 / OUTPUT_LEVEL_LADDER_RESISTANCE_OHMS);

/// Loaded small-signal transfer of either VR1 track. The service schematic
/// identifies a B taper (linear resistance distribution); the roughly 994 Ohm
/// following load turns that track into the instrument's acoustic volume law.
pub fn bender_board_volume_gain(control: f32) -> f32 {
    let position = control.clamp(0.0, 1.0);
    if position <= 0.0 {
        return 0.0;
    }
    if position >= 1.0 {
        return 1.0;
    }
    let upper = BENDER_VOLUME_TRACK_RESISTANCE_OHMS * (1.0 - position);
    let lower = BENDER_VOLUME_TRACK_RESISTANCE_OHMS * position;
    let loaded_lower = 1.0 / (1.0 / lower + 1.0 / BENDER_VOLUME_LOAD_RESISTANCE_OHMS);
    loaded_lower / (upper + loaded_lower)
}

/// Applies the nominal matched dual-gang control after the stereo chorus.
pub fn bender_board_volume(stereo: (f32, f32), control: f32) -> (f32, f32) {
    let gain = bender_board_volume_gain(control);
    (stereo.0 * gain, stereo.1 * gain)
}

pub fn module_voice_sum(voices: &[f32]) -> f32 {
    voices.iter().copied().sum::<f32>() * VOICE_SUM_GAIN_PER_VOICE
}

/// Nominal large-signal transfer used beyond the M5218's guaranteed output
/// range. The data book establishes the linear envelope and typical swing but
/// does not publish a clipping equation, so the C1-continuous tanh knee between
/// those two anchors remains an explicit E5 modeling choice.
pub fn nominal_m5218_transfer(input: f32) -> f32 {
    let magnitude = input.abs();
    if magnitude <= M5218_LINEAR_PEAK_UNITS {
        return input;
    }
    let span = M5218_TYP_PEAK_UNITS - M5218_LINEAR_PEAK_UNITS;
    input.signum()
        * (M5218_LINEAR_PEAK_UNITS
            + span * libm::tanhf((magnitude - M5218_LINEAR_PEAK_UNITS) / span))
}

#[derive(Clone, Copy, Debug, Default)]
pub struct M5218Summer {
    output: f32,
}

impl M5218Summer {
    pub const fn new() -> Self {
        Self { output: 0.0 }
    }

    pub fn reset(&mut self) {
        self.output = 0.0;
    }

    pub fn process(&mut self, voices: &[f32], sample_rate: f32) -> f32 {
        self.process_ideal_sum(module_voice_sum(voices), sample_rate)
    }

    pub fn process_ideal_sum(&mut self, ideal_sum: f32, sample_rate: f32) -> f32 {
        let target = nominal_m5218_transfer(ideal_sum);
        let volts_per_second = M5218_SLEW_RATE_VOLTS_PER_US * 1_000_000.0;
        let maximum_step = volts_per_second / (VOICE_UNIT_PEAK_VOLTS * sample_rate.max(1.0));
        let delta = target - self.output;
        if delta.abs() <= maximum_step {
            self.output = target;
        } else {
            self.output += delta.signum() * maximum_step;
        }
        self.output
    }

    pub const fn output(&self) -> f32 {
        self.output
    }
}

/// Front-panel order, from the bottom position upwards. B_2 writes the
/// complementary binary value to IC40; keeping panel order here prevents the
/// active-low hardware encoding from leaking into audio code.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum HpfPosition {
    BassBoost = 0,
    #[default]
    Flat = 1,
    LowCut = 2,
    HighCut = 3,
}

impl HpfPosition {
    pub const fn from_native(value: f64) -> Self {
        match value as u8 {
            0 => Self::BassBoost,
            2 => Self::LowCut,
            3 => Self::HighCut,
            _ => Self::Flat,
        }
    }

    /// IC40 bits observed after B_2 decodes the switch-2 serial byte.
    pub const fn ic40_bits(self) -> u8 {
        match self {
            Self::BassBoost => 0x18,
            Self::Flat => 0x10,
            Self::LowCut => 0x08,
            Self::HighCut => 0x00,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct OnePoleHighPass {
    low_state: f32,
    sample_rate: f32,
    cutoff_hz: f32,
    coefficient: f32,
}

impl OnePoleHighPass {
    fn reset(&mut self) {
        self.low_state = 0.0;
    }

    fn process(&mut self, input: f32, sample_rate: f32, cutoff_hz: f32) -> f32 {
        if self.sample_rate.to_bits() != sample_rate.to_bits()
            || self.cutoff_hz.to_bits() != cutoff_hz.to_bits()
        {
            self.coefficient = libm::tanf(PI * cutoff_hz / sample_rate).clamp(0.0, 1.0);
            self.sample_rate = sample_rate;
            self.cutoff_hz = cutoff_hz;
        }
        let g = self.coefficient;
        let high = (input - self.low_state) / (1.0 + g);
        self.low_state += 2.0 * g * high;
        high
    }
}

/// Small-signal transfer of the M5218L bass circuit on the Jack Board. The
/// coefficients are rebuilt from schematic component values, rather than from
/// a fitted response table.
#[derive(Clone, Copy, Debug)]
struct BassBoost {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    z1: f32,
    z2: f32,
    sample_rate: f32,
}

impl BassBoost {
    const fn new() -> Self {
        Self {
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            z1: 0.0,
            z2: 0.0,
            sample_rate: 0.0,
        }
    }

    fn reset(&mut self) {
        self.z1 = 0.0;
        self.z2 = 0.0;
    }

    fn prepare(&mut self, sample_rate: f32) {
        if self.sample_rate == sample_rate {
            return;
        }
        // Jack Board values: first passive network 47k/.047u/.01u;
        // non-inverting stage 10k/100k/.022u; summer 47k/220k/47k.
        const R1: f32 = 47_000.0;
        const C1: f32 = 0.047e-6;
        const CA: f32 = 0.01e-6;
        const RG: f32 = 10_000.0;
        const RF: f32 = 100_000.0;
        const CF: f32 = 0.022e-6;
        const R_DIRECT: f32 = 47_000.0;
        const R_BOOST: f32 = 220_000.0;
        const R_FEEDBACK: f32 = 47_000.0;

        let first_zero = R1 * C1;
        let first_pole = R1 * (C1 + CA);
        let second_zero = (RG * RF / (RG + RF)) * CF;
        let second_pole = RF * CF;
        let stage_gain = 1.0 + RF / RG;
        let direct_gain = R_FEEDBACK / R_DIRECT;
        let boost_gain = R_FEEDBACK / R_BOOST * stage_gain;

        let denominator = [1.0, first_pole + second_pole, first_pole * second_pole];
        let boost_numerator = [1.0, first_zero + second_zero, first_zero * second_zero];
        let numerator = [
            direct_gain * denominator[0] + boost_gain * boost_numerator[0],
            direct_gain * denominator[1] + boost_gain * boost_numerator[1],
            direct_gain * denominator[2] + boost_gain * boost_numerator[2],
        ];

        let k = 2.0 * sample_rate;
        let k2 = k * k;
        let normalizer = denominator[0] + denominator[1] * k + denominator[2] * k2;
        self.b0 = (numerator[0] + numerator[1] * k + numerator[2] * k2) / normalizer;
        self.b1 = 2.0 * (numerator[0] - numerator[2] * k2) / normalizer;
        self.b2 = (numerator[0] - numerator[1] * k + numerator[2] * k2) / normalizer;
        self.a1 = 2.0 * (denominator[0] - denominator[2] * k2) / normalizer;
        self.a2 = (denominator[0] - denominator[1] * k + denominator[2] * k2) / normalizer;
        self.sample_rate = sample_rate;
        self.reset();
    }

    fn process(&mut self, input: f32) -> f32 {
        let output = self.b0 * input + self.z1;
        self.z1 = self.b1 * input - self.a1 * output + self.z2;
        self.z2 = self.b2 * input - self.a2 * output;
        output
    }
}

#[derive(Clone, Copy, Debug)]
pub struct JackBoardHpf {
    low_cut: OnePoleHighPass,
    high_cut: OnePoleHighPass,
    bass_boost: BassBoost,
}

impl JackBoardHpf {
    pub const fn new() -> Self {
        Self {
            low_cut: OnePoleHighPass {
                low_state: 0.0,
                sample_rate: 0.0,
                cutoff_hz: 0.0,
                coefficient: 0.0,
            },
            high_cut: OnePoleHighPass {
                low_state: 0.0,
                sample_rate: 0.0,
                cutoff_hz: 0.0,
                coefficient: 0.0,
            },
            bass_boost: BassBoost::new(),
        }
    }

    pub fn reset(&mut self) {
        self.low_cut.reset();
        self.high_cut.reset();
        self.bass_boost.reset();
    }

    pub fn process(&mut self, input: f32, sample_rate: f32, position: HpfPosition) -> f32 {
        // Keep every path warm so a hardware switch change does not resurrect
        // stale filter state. The electrical switching transient remains a
        // future measurement item.
        self.bass_boost.prepare(sample_rate);
        let bass = self.bass_boost.process(input);
        let low = self.low_cut.process(input, sample_rate, HPF_LOW_CUTOFF_HZ);
        let high = self
            .high_cut
            .process(input, sample_rate, HPF_HIGH_CUTOFF_HZ);
        match position {
            HpfPosition::BassBoost => bass,
            HpfPosition::Flat => input,
            HpfPosition::LowCut => low,
            HpfPosition::HighCut => high,
        }
    }
}

impl Default for JackBoardHpf {
    fn default() -> Self {
        Self::new()
    }
}

/// B_2 sends the seven-bit VCA LEVEL parameter to physical DAC selector D6 as
/// `raw << 5`. The DAC is a 12-bit R-2R ladder referenced to +5 V.
pub const OUTPUT_VCA_DAC_COUNTS_PER_PANEL_STEP: u16 = 32;
pub const OUTPUT_VCA_DAC_DIVISOR: f32 = 4096.0;
pub const OUTPUT_VCA_DAC_REFERENCE_VOLTS: f32 = 5.0;

/// Module Board IC27a converts the positive DAC voltage at TP4 to the negative
/// VCA LEVEL rail at TP3 through the documented 4.99 kOhm / 10 kOhm pair.
pub const OUTPUT_VCA_DAC_INVERT_INPUT_OHMS: f32 = 4_990.0;
pub const OUTPUT_VCA_DAC_INVERT_FEEDBACK_OHMS: f32 = 10_000.0;

/// Jack Board control network around C7 and uPC1252H2 GC1.
pub const OUTPUT_VCA_R30_OHMS: f32 = 2_200.0;
pub const OUTPUT_VCA_R32_OHMS: f32 = 1_500.0;
pub const OUTPUT_VCA_R31_OHMS: f32 = 47.0;
pub const OUTPUT_VCA_R165_OHMS: f32 = 15_000.0;
pub const OUTPUT_VCA_C7_FARADS: f32 = 10.0e-6;
pub const OUTPUT_VCA_POSITIVE_SUPPLY_VOLTS: f32 = 15.0;

/// NEC characterizes GC1 at -5.9 mV/dB typical over -30..+30 dB.
pub const UPC1252H2_CONTROL_VOLTS_PER_DB: f32 = -5.9e-3;
pub const UPC1252H2_CHARACTERIZED_MIN_GAIN_DB: f32 = -30.0;
pub const UPC1252H2_CHARACTERIZED_MAX_GAIN_DB: f32 = 30.0;
pub const UPC1252H2_INPUT_MIN_DBV: f32 = -40.0;
pub const UPC1252H2_INPUT_MAX_DBV: f32 = 10.0;
pub const UPC1252H2_TYPICAL_THD_UNITY: f32 = 0.000_07;
pub const UPC1252H2_MAX_THD_UNITY: f32 = 0.000_7;
pub const UPC1252H2_TYPICAL_OUTPUT_NOISE_DBV: f32 = -94.0;
pub const UPC1252H2_MAX_OUTPUT_NOISE_DBV: f32 = -84.0;

const OUTPUT_VCA_SHUNT_OHMS: f32 = 1.0 / (1.0 / OUTPUT_VCA_R31_OHMS + 1.0 / OUTPUT_VCA_R165_OHMS);
pub const OUTPUT_VCA_C7_EFFECTIVE_RESISTANCE_OHMS: f32 =
    1.0 / (1.0 / OUTPUT_VCA_R30_OHMS + 1.0 / (OUTPUT_VCA_R32_OHMS + OUTPUT_VCA_SHUNT_OHMS));
pub const OUTPUT_VCA_CONTROL_TAU_SECONDS: f32 =
    OUTPUT_VCA_C7_EFFECTIVE_RESISTANCE_OHMS * OUTPUT_VCA_C7_FARADS;

pub fn output_vca_panel_raw(control: f32) -> u8 {
    libm::roundf(control.clamp(0.0, 1.0) * 127.0) as u8
}

pub fn output_vca_dac_word(control: f32) -> u16 {
    u16::from(output_vca_panel_raw(control)) * OUTPUT_VCA_DAC_COUNTS_PER_PANEL_STEP
}

pub fn output_vca_dac_voltage(control: f32) -> f32 {
    OUTPUT_VCA_DAC_REFERENCE_VOLTS * f32::from(output_vca_dac_word(control))
        / OUTPUT_VCA_DAC_DIVISOR
}

pub fn output_vca_level_voltage(control: f32) -> f32 {
    -output_vca_dac_voltage(control) * OUTPUT_VCA_DAC_INVERT_FEEDBACK_OHMS
        / OUTPUT_VCA_DAC_INVERT_INPUT_OHMS
}

/// Static GC1 voltage after R30 + R32, the 47 Ohm shunt and the 15 kOhm bias.
pub fn output_vca_gc1_voltage(control: f32) -> f32 {
    let series = OUTPUT_VCA_R30_OHMS + OUTPUT_VCA_R32_OHMS;
    (output_vca_level_voltage(control) / series
        + OUTPUT_VCA_POSITIVE_SUPPLY_VOLTS / OUTPUT_VCA_R165_OHMS)
        / (1.0 / series + 1.0 / OUTPUT_VCA_R31_OHMS + 1.0 / OUTPUT_VCA_R165_OHMS)
}

pub fn output_vca_gain_db(control: f32) -> f32 {
    output_vca_gc1_voltage(control) / UPC1252H2_CONTROL_VOLTS_PER_DB
}

pub fn output_vca_gain(control: f32) -> f32 {
    libm::powf(10.0, output_vca_gain_db(control) / 20.0)
}

fn output_vca_capacitor_target(control: f32) -> f32 {
    let level = output_vca_level_voltage(control);
    let shunt_source = OUTPUT_VCA_POSITIVE_SUPPLY_VOLTS * OUTPUT_VCA_R31_OHMS
        / (OUTPUT_VCA_R31_OHMS + OUTPUT_VCA_R165_OHMS);
    let current = (level - shunt_source)
        / (OUTPUT_VCA_R30_OHMS + OUTPUT_VCA_R32_OHMS + OUTPUT_VCA_SHUNT_OHMS);
    level - current * OUTPUT_VCA_R30_OHMS
}

fn output_vca_gc1_from_capacitor(capacitor_voltage: f32) -> f32 {
    (capacitor_voltage / OUTPUT_VCA_R32_OHMS
        + OUTPUT_VCA_POSITIVE_SUPPLY_VOLTS / OUTPUT_VCA_R165_OHMS)
        / (1.0 / OUTPUT_VCA_R32_OHMS + 1.0 / OUTPUT_VCA_R31_OHMS + 1.0 / OUTPUT_VCA_R165_OHMS)
}

#[derive(Clone, Copy, Debug)]
pub struct Upc1252H2Vca {
    capacitor_voltage: f32,
    control_voltage: f32,
    gain: f32,
}

impl Upc1252H2Vca {
    pub const fn new() -> Self {
        // -1.5 V at C7 makes GC1 exactly 0 V through R32 and R165, preserving
        // a click-free deterministic reset before the first panel value lands.
        Self {
            capacitor_voltage: -1.5,
            control_voltage: 0.0,
            gain: 1.0,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn process_control(&mut self, control: f32, sample_rate: f32) -> f32 {
        let target = output_vca_capacitor_target(control);
        let coefficient =
            1.0 - libm::expf(-1.0 / (sample_rate.max(1.0) * OUTPUT_VCA_CONTROL_TAU_SECONDS));
        self.capacitor_voltage += (target - self.capacitor_voltage) * coefficient;
        self.control_voltage = output_vca_gc1_from_capacitor(self.capacitor_voltage);
        let gain_db = self.control_voltage / UPC1252H2_CONTROL_VOLTS_PER_DB;
        self.gain = libm::powf(10.0, gain_db / 20.0);
        self.gain
    }

    pub const fn gain(&self) -> f32 {
        self.gain
    }

    pub const fn control_voltage(&self) -> f32 {
        self.control_voltage
    }
}

impl Default for Upc1252H2Vca {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OutputPath {
    hpf: JackBoardHpf,
    vca: Upc1252H2Vca,
    vca_output: M5218Summer,
}

impl OutputPath {
    pub const fn new() -> Self {
        Self {
            hpf: JackBoardHpf::new(),
            vca: Upc1252H2Vca::new(),
            vca_output: M5218Summer::new(),
        }
    }

    pub fn reset(&mut self) {
        self.hpf.reset();
        self.vca.reset();
        self.vca_output.reset();
    }

    pub fn process(
        &mut self,
        voice_sum: f32,
        sample_rate: f32,
        hpf_position: HpfPosition,
        vca_control: f32,
    ) -> f32 {
        let filtered = self.hpf.process(voice_sum, sample_rate, hpf_position);
        let gain = self.vca.process_control(vca_control, sample_rate);
        self.vca_output
            .process_ideal_sum(filtered * gain, sample_rate)
    }

    pub const fn vca_gain(&self) -> f32 {
        self.vca.gain()
    }

    pub const fn vca_control_voltage(&self) -> f32 {
        self.vca.control_voltage()
    }
}

impl Default for OutputPath {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switched_cutoffs_follow_schematic_components() {
        assert!((HPF_LOW_CUTOFF_HZ - 236.35).abs() < 0.1);
        assert!((HPF_HIGH_CUTOFF_HZ - 754.31).abs() < 0.2);
    }

    #[test]
    fn module_summer_uses_the_documented_33k_to_3k3_ratio() {
        assert!((VOICE_SUM_GAIN_PER_VOICE + 0.1).abs() < 1.0e-7);
        assert!((module_voice_sum(&[1.0]) + 0.1).abs() < 1.0e-7);
        assert!((module_voice_sum(&[1.0; 6]) + 0.6).abs() < 1.0e-6);
        assert_eq!(module_voice_sum(&[1.0, -1.0]), 0.0);
    }

    #[test]
    fn bender_volume_uses_the_dual_10k_tracks_and_nominal_jack_board_load() {
        assert!((BENDER_VOLUME_LOAD_RESISTANCE_OHMS - 993.670_9).abs() < 0.001);
        assert_eq!(bender_board_volume_gain(0.0), 0.0);
        assert_eq!(bender_board_volume_gain(1.0), 1.0);
        assert!((bender_board_volume_gain(0.5) - 0.142_210_14).abs() < 1.0e-6);

        let mut previous = 0.0;
        for step in 1..=100 {
            let gain = bender_board_volume_gain(step as f32 / 100.0);
            assert!(gain > previous);
            previous = gain;
        }
    }

    #[test]
    fn matched_volume_tracks_preserve_the_stereo_relationship() {
        let (left, right) = bender_board_volume((0.25, -0.5), 0.73);
        assert_eq!(right, -2.0 * left);
        assert_eq!(bender_board_volume((1.0, -1.0), -1.0), (0.0, -0.0));
        assert_eq!(bender_board_volume((1.0, -1.0), 2.0), (1.0, -1.0));
    }

    #[test]
    fn m5218_is_linear_through_the_guaranteed_output_swing() {
        for input in [-4.0, -1.0, 0.0, 1.0, 4.0] {
            assert_eq!(nominal_m5218_transfer(input), input);
        }
    }

    #[test]
    fn m5218_nominal_knee_is_symmetric_and_bounded_by_typical_swing() {
        for input in [4.1, 5.0, 10.0, 100.0] {
            let positive = nominal_m5218_transfer(input);
            let negative = nominal_m5218_transfer(-input);
            assert_eq!(negative, -positive);
            assert!(positive >= M5218_LINEAR_PEAK_UNITS);
            assert!(positive <= M5218_TYP_PEAK_UNITS);
        }
        assert!(nominal_m5218_transfer(100.0) > M5218_TYP_PEAK_UNITS - 1.0e-5);
    }

    #[test]
    fn m5218_slew_uses_the_contemporary_two_point_two_volts_per_microsecond() {
        let mut summer = M5218Summer::new();
        let sample_rate = 192_000.0;
        let expected_step =
            M5218_SLEW_RATE_VOLTS_PER_US * 1_000_000.0 / (VOICE_UNIT_PEAK_VOLTS * sample_rate);
        let first = summer.process_ideal_sum(100.0, sample_rate);
        assert!((first - expected_step).abs() < 1.0e-6);
        assert_eq!(summer.output(), first);
        summer.reset();
        assert_eq!(summer.output(), 0.0);
    }

    #[test]
    fn front_panel_positions_reconstruct_active_low_ic40_bits() {
        assert_eq!(HpfPosition::BassBoost.ic40_bits(), 0x18);
        assert_eq!(HpfPosition::Flat.ic40_bits(), 0x10);
        assert_eq!(HpfPosition::LowCut.ic40_bits(), 0x08);
        assert_eq!(HpfPosition::HighCut.ic40_bits(), 0x00);
    }

    #[test]
    fn output_vca_follows_firmware_dac_and_jack_board_components() {
        assert_eq!(output_vca_dac_word(0.0), 0x000);
        assert_eq!(output_vca_dac_word(64.0 / 127.0), 0x800);
        assert_eq!(output_vca_dac_word(1.0), 0xfe0);
        assert!((output_vca_gain_db(0.0) + 7.841_917).abs() < 1.0e-4);
        assert!((output_vca_gain_db(64.0 / 127.0) - 2.776_505).abs() < 1.0e-4);
        assert!((output_vca_gain_db(1.0) - 13.229_014).abs() < 1.0e-4);
        assert!(output_vca_gain_db(47.0 / 127.0) < 0.0);
        assert!(output_vca_gain_db(48.0 / 127.0) > 0.0);
    }

    #[test]
    fn output_vca_control_filter_comes_from_c7_network() {
        assert!((OUTPUT_VCA_C7_EFFECTIVE_RESISTANCE_OHMS - 908.249_4).abs() < 0.01);
        assert!((OUTPUT_VCA_CONTROL_TAU_SECONDS - 0.009_082_494).abs() < 1.0e-7);
        let mut vca = Upc1252H2Vca::new();
        for _ in 0..48_000 {
            vca.process_control(1.0, 48_000.0);
        }
        assert!(
            (vca.gain() - output_vca_gain(1.0)).abs() < 5.0e-4,
            "dynamic={} static={} control={} static_control={}",
            vca.gain(),
            output_vca_gain(1.0),
            vca.control_voltage(),
            output_vca_gc1_voltage(1.0)
        );
        assert!((vca.control_voltage() - output_vca_gc1_voltage(1.0)).abs() < 5.0e-6);
    }

    #[test]
    fn all_hpf_paths_remain_finite() {
        for position in [
            HpfPosition::BassBoost,
            HpfPosition::Flat,
            HpfPosition::LowCut,
            HpfPosition::HighCut,
        ] {
            let mut filter = JackBoardHpf::new();
            for sample in 0..96_000 {
                let input = if sample == 0 { 1.0 } else { 0.0 };
                let output = filter.process(input, 48_000.0, position);
                assert!(output.is_finite());
            }
        }
    }

    #[test]
    fn fixed_hpf_coefficient_cache_is_sample_exact() {
        let mut cached = JackBoardHpf::new();
        for sample in 0..64 {
            let _ = cached.process(sample as f32 / 64.0, 48_000.0, HpfPosition::LowCut);
        }
        let mut rebuilt = cached;
        rebuilt.low_cut.sample_rate = 0.0;
        rebuilt.high_cut.sample_rate = 0.0;

        let cached_output = cached.process(-0.25, 48_000.0, HpfPosition::HighCut);
        let rebuilt_output = rebuilt.process(-0.25, 48_000.0, HpfPosition::HighCut);
        assert_eq!(cached_output.to_bits(), rebuilt_output.to_bits());
    }
}
