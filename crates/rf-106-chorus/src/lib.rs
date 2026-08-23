#![no_std]

use core::f32::consts::PI;
#[cfg(test)]
use core::f32::consts::TAU;
use rf_106_output::VOICE_UNIT_PEAK_VOLTS;

pub const MN3009_STAGES: usize = 256;
pub const MN3009_MIN_CLOCK_HZ: f32 = 10_000.0;
pub const MN3009_MAX_CLOCK_HZ: f32 = 200_000.0;
pub const MN3009_MIN_DELAY_MS: f32 = MN3009_STAGES as f32 * 1_000.0 / (2.0 * MN3009_MAX_CLOCK_HZ);
pub const MN3009_MAX_DELAY_MS: f32 = MN3009_STAGES as f32 * 1_000.0 / (2.0 * MN3009_MIN_CLOCK_HZ);
/// The manufacturer specifies a 14 kHz, -3 dB input bandwidth at a 40 kHz
/// BBD clock. This ratio is an explicit nominal device property rather than a
/// calibration of any one surviving unit.
pub const MN3009_NOMINAL_BANDWIDTH_TO_CLOCK: f32 = 14_000.0 / 40_000.0;
/// Manufacturer test point for the nominal total harmonic distortion figure.
pub const MN3009_THD_TEST_INPUT_VRMS: f32 = 0.78;
/// Typical total harmonic distortion at 40 kHz clock and 1 kHz input.
pub const MN3009_TYPICAL_THD: f32 = 0.003;
/// Manufacturer input-swing point defined at 2.5% THD.
pub const MN3009_INPUT_SWING_VRMS: f32 = 1.7;
pub const MN3009_INPUT_SWING_THD: f32 = 0.025;
/// Parameters of the bounded odd-symmetric transfer in physical peak volts.
/// They are solved jointly from the 0.78 Vrms / 0.3% typical point and the
/// 1.7 Vrms / 2.5% input-swing point. This remains a nominal device model, not
/// a tolerance claim for every MN3009.
pub const MN3009_NOMINAL_TRANSFER_SCALE_VOLTS: f32 = 2.981_463_7;
pub const MN3009_NOMINAL_TRANSFER_CUBIC: f32 = 0.268_241_4;
const DELAY_CAPACITY: usize = 2048;
const MODE_FADE_SECONDS: f32 = 0.005;

/// Nominal memoryless BBD transfer at the documented operating point.
/// RF-106 signal units retain the physical 3 V-peak convention established by
/// the service VCA adjustment, so the datasheet voltage can be applied without
/// an arbitrary normalized-audio drive constant.
pub fn mn3009_nominal_transfer(input: f32) -> f32 {
    let input_volts = input * VOICE_UNIT_PEAK_VOLTS;
    let normalized = input_volts / MN3009_NOMINAL_TRANSFER_SCALE_VOLTS;
    let shaped = normalized + MN3009_NOMINAL_TRANSFER_CUBIC * normalized * normalized * normalized;
    let output_volts = MN3009_NOMINAL_TRANSFER_SCALE_VOLTS * libm::tanhf(shaped);
    output_volts / VOICE_UNIT_PEAK_VOLTS
}

/// These values define a replaceable calibration layer, not established
/// RF-106 constants. They are retained as explicit E6 hypotheses until raw
/// captures from identified hardware can be archived and reproduced.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChorusCalibration {
    pub center_delay_ms: f32,
    pub chorus_i_rate_hz: f32,
    pub chorus_i_depth_ms: f32,
    pub chorus_ii_rate_hz: f32,
    pub chorus_ii_depth_ms: f32,
    pub filter_cutoff_hz: f32,
    pub dry_gain: f32,
    pub wet_gain: f32,
}

pub const PROVISIONAL_CALIBRATION: ChorusCalibration = ChorusCalibration {
    center_delay_ms: 3.30,
    chorus_i_rate_hz: 0.514,
    chorus_i_depth_ms: 2.13,
    chorus_ii_rate_hz: 0.842,
    chorus_ii_depth_ms: 1.71,
    filter_cutoff_hz: 9_661.0,
    // IC6 resistor-domain hypothesis: the wet/dry ratio is 47K/39K.
    dry_gain: 1.0,
    wet_gain: 47.0 / 39.0,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ChorusMode {
    #[default]
    Off,
    I,
    II,
}

impl ChorusMode {
    pub const fn from_switches(chorus_i: bool, chorus_ii: bool) -> Self {
        match (chorus_i, chorus_ii) {
            (false, false) => Self::Off,
            (true, false) => Self::I,
            (false, true) => Self::II,
            // Legacy RF-106 state could set two synthetic booleans at once.
            // The original panel has three momentary mode buttons. Its
            // firmware maps a coincident I+II scan to the same Module Board
            // control lines as mode I.
            (true, true) => Self::I,
        }
    }

    /// Assigner switch-1 bits reconstructed from the verified A_5 firmware.
    /// The three front-panel buttons occupy bits 5, 6 and 7 respectively.
    pub const fn switch1_bits(self) -> u8 {
        match self {
            Self::Off => 0x20,
            Self::I => 0x40,
            Self::II => 0x80,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct OnePoleLowPass {
    state: f32,
}

impl OnePoleLowPass {
    fn reset(&mut self) {
        self.state = 0.0;
    }

    fn process(&mut self, input: f32, sample_rate: f32, cutoff_hz: f32) -> f32 {
        let cutoff = cutoff_hz.clamp(10.0, sample_rate * 0.45);
        let g = libm::tanf(PI * cutoff / sample_rate);
        let delta = (input - self.state) * g / (1.0 + g);
        let output = self.state + delta;
        self.state = output + delta;
        output
    }
}

#[derive(Clone, Copy)]
struct Mn3009Line {
    buffer: [f32; DELAY_CAPACITY],
    write: usize,
    pre_filter: [OnePoleLowPass; 2],
    bbd_bandwidth: OnePoleLowPass,
    post_filter: [OnePoleLowPass; 2],
}

impl Mn3009Line {
    const fn new() -> Self {
        Self {
            buffer: [0.0; DELAY_CAPACITY],
            write: 0,
            pre_filter: [OnePoleLowPass { state: 0.0 }, OnePoleLowPass { state: 0.0 }],
            bbd_bandwidth: OnePoleLowPass { state: 0.0 },
            post_filter: [OnePoleLowPass { state: 0.0 }, OnePoleLowPass { state: 0.0 }],
        }
    }

    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write = 0;
        for filter in &mut self.pre_filter {
            filter.reset();
        }
        self.bbd_bandwidth.reset();
        for filter in &mut self.post_filter {
            filter.reset();
        }
    }

    fn process(&mut self, input: f32, delay_samples: f32, sample_rate: f32, cutoff_hz: f32) -> f32 {
        let mut filtered = input;
        for filter in &mut self.pre_filter {
            filtered = filter.process(filtered, sample_rate, cutoff_hz);
        }
        // Apply the device's nominal distortion in physical volts before the
        // bucket-brigade delay. Bias asymmetry and unit variance remain future
        // capture items; neither is invented by this symmetric transfer.
        let stored = mn3009_nominal_transfer(filtered);
        self.buffer[self.write] = stored;

        let delay = delay_samples.clamp(2.0, (DELAY_CAPACITY - 3) as f32);
        let position = self.write as f32 - delay;
        let wrapped = if position < 0.0 {
            position + DELAY_CAPACITY as f32
        } else {
            position
        };
        let index = wrapped as usize;
        let fraction = wrapped - index as f32;
        let sample = hermite(
            fraction,
            self.buffer[(index + DELAY_CAPACITY - 1) & (DELAY_CAPACITY - 1)],
            self.buffer[index & (DELAY_CAPACITY - 1)],
            self.buffer[(index + 1) & (DELAY_CAPACITY - 1)],
            self.buffer[(index + 2) & (DELAY_CAPACITY - 1)],
        );
        self.write = (self.write + 1) & (DELAY_CAPACITY - 1);

        // A 256-stage BBD's intrinsic bandwidth follows its clock. Keep this
        // separate from the fixed board-level anti-alias/reconstruction
        // filters so slow portions of the modulation become slightly darker,
        // as specified by the original device data.
        let clock_hz = MN3009_STAGES as f32 * sample_rate / (2.0 * delay_samples);
        let bbd_cutoff_hz = StereoChorus::nominal_bandwidth_hz_for_clock(clock_hz);
        let mut reconstructed = self
            .bbd_bandwidth
            .process(sample, sample_rate, bbd_cutoff_hz);
        for filter in &mut self.post_filter {
            reconstructed = filter.process(reconstructed, sample_rate, cutoff_hz);
        }
        reconstructed
    }
}

fn hermite(fraction: f32, y0: f32, y1: f32, y2: f32, y3: f32) -> f32 {
    let c0 = y1;
    let c1 = 0.5 * (y2 - y0);
    let c2 = y0 - 2.5 * y1 + 2.0 * y2 - 0.5 * y3;
    let c3 = 0.5 * (y3 - y0) + 1.5 * (y1 - y2);
    ((c3 * fraction + c2) * fraction + c1) * fraction + c0
}

#[derive(Clone, Copy)]
pub struct StereoChorus {
    lines: [Mn3009Line; 2],
    phase: f32,
    mode: ChorusMode,
    pending_mode: ChorusMode,
    wet: f32,
    wet_target: f32,
    calibration: ChorusCalibration,
}

impl StereoChorus {
    pub const fn new() -> Self {
        Self::with_calibration(PROVISIONAL_CALIBRATION)
    }

    pub const fn with_calibration(calibration: ChorusCalibration) -> Self {
        Self {
            lines: [Mn3009Line::new(), Mn3009Line::new()],
            phase: 0.0,
            mode: ChorusMode::Off,
            pending_mode: ChorusMode::Off,
            wet: 0.0,
            wet_target: 0.0,
            calibration,
        }
    }

    pub fn reset(&mut self) {
        for line in &mut self.lines {
            line.reset();
        }
        self.phase = 0.0;
        self.mode = ChorusMode::Off;
        self.pending_mode = ChorusMode::Off;
        self.wet = 0.0;
        self.wet_target = 0.0;
    }

    pub const fn mode(&self) -> ChorusMode {
        self.mode
    }

    pub const fn calibration(&self) -> ChorusCalibration {
        self.calibration
    }

    pub fn set_mode(&mut self, mode: ChorusMode) {
        if mode == self.pending_mode {
            return;
        }
        self.pending_mode = mode;
        if mode == self.mode {
            // The buttons returned to the active mode while a mode-to-mode
            // fade was in flight: cancel the pending switch and fade back in.
            self.wet_target = if mode == ChorusMode::Off { 0.0 } else { 1.0 };
            return;
        }
        if self.mode == ChorusMode::Off || mode == ChorusMode::Off {
            self.mode = mode;
            self.wet_target = if mode == ChorusMode::Off { 0.0 } else { 1.0 };
        } else {
            // A live mode-to-mode transition changes two clock networks. Fade
            // the wet bus out before changing their calibration.
            self.wet_target = 0.0;
        }
    }

    pub fn process(&mut self, input: f32, sample_rate: f32) -> (f32, f32) {
        let fade_step = 1.0 / (sample_rate * MODE_FADE_SECONDS).max(1.0);
        if self.wet < self.wet_target {
            self.wet = (self.wet + fade_step).min(self.wet_target);
        } else if self.wet > self.wet_target {
            self.wet = (self.wet - fade_step).max(self.wet_target);
        }
        if self.wet == 0.0 && self.mode != self.pending_mode {
            self.mode = self.pending_mode;
            self.wet_target = if self.mode == ChorusMode::Off {
                0.0
            } else {
                1.0
            };
        }

        let (rate_hz, depth_ms) = match self.mode {
            ChorusMode::Off => (self.calibration.chorus_i_rate_hz, 0.0),
            ChorusMode::I => (
                self.calibration.chorus_i_rate_hz,
                self.calibration.chorus_i_depth_ms,
            ),
            ChorusMode::II => (
                self.calibration.chorus_ii_rate_hz,
                self.calibration.chorus_ii_depth_ms,
            ),
        };
        self.phase = (self.phase + rate_hz / sample_rate) % 1.0;
        // All switch combinations drive the same free-running integrator.
        // TP3 and TP4 are buffered inversions of this triangle; the switch
        // network changes rate/depth, not oscillator family.
        let modulation = triangle_modulation(self.phase);

        let left_ms = Self::clamp_delay_ms_to_clock_envelope(
            self.calibration.center_delay_ms + depth_ms * modulation,
        );
        let right_ms = Self::clamp_delay_ms_to_clock_envelope(
            self.calibration.center_delay_ms - depth_ms * modulation,
        );
        let left_wet = self.lines[0].process(
            input,
            left_ms * sample_rate * 0.001,
            sample_rate,
            self.calibration.filter_cutoff_hz,
        );
        let right_wet = self.lines[1].process(
            input,
            right_ms * sample_rate * 0.001,
            sample_rate,
            self.calibration.filter_cutoff_hz,
        );

        if self.wet == 0.0 {
            return (input, input);
        }
        let dry_gain = 1.0 - self.wet * (1.0 - self.calibration.dry_gain);
        let wet_gain = self.wet * self.calibration.wet_gain;
        (
            input * dry_gain + left_wet * wet_gain,
            input * dry_gain + right_wet * wet_gain,
        )
    }

    pub fn clock_hz_for_delay_ms(delay_ms: f32) -> f32 {
        MN3009_STAGES as f32 / (2.0 * delay_ms * 0.001)
    }

    pub fn delay_ms_for_clock_hz(clock_hz: f32) -> f32 {
        MN3009_STAGES as f32 * 1_000.0 / (2.0 * clock_hz)
    }

    pub fn nominal_bandwidth_hz_for_clock(clock_hz: f32) -> f32 {
        clock_hz * MN3009_NOMINAL_BANDWIDTH_TO_CLOCK
    }

    pub fn clamp_delay_ms_to_clock_envelope(delay_ms: f32) -> f32 {
        delay_ms.clamp(MN3009_MIN_DELAY_MS, MN3009_MAX_DELAY_MS)
    }
}

fn triangle_modulation(phase: f32) -> f32 {
    1.0 - 4.0 * (phase - 0.5).abs()
}

impl Default for StereoChorus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_dual_boolean_state_canonicalizes_to_the_firmware_mode() {
        assert_eq!(ChorusMode::from_switches(false, false), ChorusMode::Off);
        assert_eq!(ChorusMode::from_switches(true, false), ChorusMode::I);
        assert_eq!(ChorusMode::from_switches(false, true), ChorusMode::II);
        assert_eq!(ChorusMode::from_switches(true, true), ChorusMode::I);
        assert_eq!(ChorusMode::Off.switch1_bits(), 0x20);
        assert_eq!(ChorusMode::I.switch1_bits(), 0x40);
        assert_eq!(ChorusMode::II.switch1_bits(), 0x80);
    }

    #[test]
    fn center_delay_and_clock_obey_the_256_stage_bbd_relation() {
        let clock = StereoChorus::clock_hz_for_delay_ms(3.30);
        assert!((clock - 38_787.88).abs() < 0.1);
    }

    #[test]
    fn delay_and_bandwidth_reproduce_the_manufacturer_points() {
        assert!((MN3009_MIN_DELAY_MS - 0.64).abs() < 1.0e-6);
        assert!((MN3009_MAX_DELAY_MS - 12.8).abs() < 1.0e-6);
        assert!((StereoChorus::delay_ms_for_clock_hz(200_000.0) - 0.64).abs() < 1.0e-6);
        assert!((StereoChorus::delay_ms_for_clock_hz(10_000.0) - 12.8).abs() < 1.0e-6);
        assert!((StereoChorus::nominal_bandwidth_hz_for_clock(40_000.0) - 14_000.0).abs() < 1.0e-3);
    }

    fn transfer_thd(input_vrms: f32) -> f32 {
        const SAMPLE_COUNT: usize = 16_384;
        let input_peak_units = input_vrms * core::f32::consts::SQRT_2 / VOICE_UNIT_PEAK_VOLTS;
        let mut real = [0.0_f32; 11];
        let mut imaginary = [0.0_f32; 11];
        for index in 0..SAMPLE_COUNT {
            let phase = TAU * index as f32 / SAMPLE_COUNT as f32;
            let output = mn3009_nominal_transfer(input_peak_units * libm::sinf(phase));
            for harmonic in 1..=10 {
                let harmonic_phase = phase * harmonic as f32;
                real[harmonic] += output * libm::cosf(harmonic_phase);
                imaginary[harmonic] -= output * libm::sinf(harmonic_phase);
            }
        }
        let amplitude = |harmonic: usize| {
            2.0 * libm::sqrtf(
                real[harmonic] * real[harmonic] + imaginary[harmonic] * imaginary[harmonic],
            ) / SAMPLE_COUNT as f32
        };
        let fundamental = amplitude(1);
        let mut harmonic_power = 0.0;
        for harmonic in 2..=10 {
            let harmonic_amplitude = amplitude(harmonic);
            harmonic_power += harmonic_amplitude * harmonic_amplitude;
        }
        libm::sqrtf(harmonic_power) / fundamental
    }

    #[test]
    fn nominal_transfer_reproduces_the_manufacturer_thd_point() {
        let thd = transfer_thd(MN3009_THD_TEST_INPUT_VRMS);
        assert!((thd - MN3009_TYPICAL_THD).abs() < 5.0e-6, "THD={thd}");
        let swing_thd = transfer_thd(MN3009_INPUT_SWING_VRMS);
        assert!(
            (swing_thd - MN3009_INPUT_SWING_THD).abs() < 5.0e-6,
            "input-swing THD={swing_thd}"
        );
    }

    #[test]
    fn nominal_transfer_is_odd_bounded_and_near_unity_at_low_level() {
        for input in [-4.0_f32, -1.0, -0.01, 0.0, 0.01, 1.0, 4.0] {
            let output = mn3009_nominal_transfer(input);
            assert!(output.is_finite());
            assert!((output + mn3009_nominal_transfer(-input)).abs() < 1.0e-6);
            assert!(output.abs() <= MN3009_NOMINAL_TRANSFER_SCALE_VOLTS / VOICE_UNIT_PEAK_VOLTS);
        }
        assert!((mn3009_nominal_transfer(0.001) / 0.001 - 1.0).abs() < 1.0e-5);
    }

    #[test]
    fn arbitrary_calibration_cannot_overclock_the_bbd_model() {
        assert_eq!(StereoChorus::clamp_delay_ms_to_clock_envelope(-10.0), 0.64);
        assert_eq!(StereoChorus::clamp_delay_ms_to_clock_envelope(1.0), 1.0);
        assert_eq!(StereoChorus::clamp_delay_ms_to_clock_envelope(100.0), 12.8);
    }

    #[test]
    fn every_calibrated_delay_stays_inside_the_bbd_clock_envelope() {
        let calibration = PROVISIONAL_CALIBRATION;
        for depth_ms in [
            calibration.chorus_i_depth_ms,
            calibration.chorus_ii_depth_ms,
        ] {
            for delay_ms in [
                calibration.center_delay_ms - depth_ms,
                calibration.center_delay_ms + depth_ms,
            ] {
                let clock_hz = StereoChorus::clock_hz_for_delay_ms(delay_ms);
                assert!(clock_hz >= MN3009_MIN_CLOCK_HZ);
                assert!(clock_hz <= MN3009_MAX_CLOCK_HZ);
            }
        }
    }

    #[test]
    fn shared_integrator_has_the_documented_triangle_turning_points() {
        assert_eq!(triangle_modulation(0.0), -1.0);
        assert_eq!(triangle_modulation(0.25), 0.0);
        assert_eq!(triangle_modulation(0.5), 1.0);
        assert_eq!(triangle_modulation(0.75), 0.0);
        assert_eq!(triangle_modulation(1.0), -1.0);
    }

    #[test]
    fn changing_active_modes_preserves_the_free_running_oscillator_phase() {
        let mut chorus = StereoChorus::new();
        chorus.set_mode(ChorusMode::I);
        for _ in 0..1000 {
            chorus.process(0.0, 48_000.0);
        }
        let phase_before_switch = chorus.phase;
        chorus.set_mode(ChorusMode::II);
        for _ in 0..500 {
            chorus.process(0.0, 48_000.0);
        }
        assert!(chorus.phase >= phase_before_switch);
        assert_ne!(chorus.phase, 0.0);
    }

    #[test]
    fn slower_bbd_clock_has_less_high_frequency_output() {
        let sample_rate = 96_000.0;
        let frequency_hz = 12_000.0;
        let mut fast = Mn3009Line::new();
        let mut slow = Mn3009Line::new();
        let fast_delay_samples = sample_rate * 0.0015;
        let slow_delay_samples = sample_rate * 0.006;
        let mut fast_energy = 0.0;
        let mut slow_energy = 0.0;
        for index in 0..48_000 {
            let input = libm::sinf(index as f32 * frequency_hz * TAU / sample_rate);
            let fast_sample = fast.process(input, fast_delay_samples, sample_rate, 40_000.0);
            let slow_sample = slow.process(input, slow_delay_samples, sample_rate, 40_000.0);
            if index > 2_000 {
                fast_energy += fast_sample * fast_sample;
                slow_energy += slow_sample * slow_sample;
            }
        }
        assert!(fast_energy > slow_energy * 1.25);
    }

    #[test]
    fn bypass_is_exact_while_bbd_lines_remain_warm() {
        let mut chorus = StereoChorus::new();
        for index in 0..4096 {
            let input = libm::sinf(index as f32 * 0.01);
            assert_eq!(chorus.process(input, 48_000.0), (input, input));
        }
    }

    #[test]
    fn every_mode_stays_finite_at_supported_sample_rates() {
        for sample_rate in [8_000.0, 44_100.0, 48_000.0, 96_000.0, 192_000.0] {
            for mode in [ChorusMode::Off, ChorusMode::I, ChorusMode::II] {
                let mut chorus = StereoChorus::new();
                chorus.set_mode(mode);
                for sample in 0..(sample_rate as usize / 2) {
                    let input = if sample == 0 { 1.0 } else { 0.0 };
                    let (left, right) = chorus.process(input, sample_rate);
                    assert!(left.is_finite() && right.is_finite());
                }
            }
        }
    }

    #[test]
    fn antiphase_delays_create_stereo_output() {
        let mut chorus = StereoChorus::new();
        chorus.set_mode(ChorusMode::I);
        let mut difference = 0.0;
        for sample in 0..48_000 {
            let input = libm::sinf(sample as f32 * 440.0 * TAU / 48_000.0);
            let (left, right) = chorus.process(input, 48_000.0);
            difference += (left - right).abs();
        }
        assert!(difference > 1.0);
    }

    #[test]
    fn returning_to_the_active_buttons_cancels_a_pending_mode_switch() {
        let mut chorus = StereoChorus::new();
        chorus.set_mode(ChorusMode::I);
        for _ in 0..1000 {
            chorus.process(0.1, 48_000.0);
        }
        chorus.set_mode(ChorusMode::II);
        for _ in 0..10 {
            chorus.process(0.1, 48_000.0);
        }
        chorus.set_mode(ChorusMode::I);
        for _ in 0..1000 {
            chorus.process(0.1, 48_000.0);
        }
        assert_eq!(chorus.mode(), ChorusMode::I);
        assert_eq!(chorus.pending_mode, ChorusMode::I);
        assert_eq!(chorus.wet, 1.0);
    }
}
