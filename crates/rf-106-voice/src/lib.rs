#![no_std]

/// Mean B_2 foreground-loop period measured from the executed cold-start
/// fixture. Individual cycles jitter around this mean as ADC interrupts land.
pub const CONTROL_CYCLE_SECONDS: f32 = 0.004_246_18;
pub const ENVELOPE_MAX: u16 = 0x3fff;
pub const DAC_MAX: u16 = 0x0fff;
/// Largest 12-bit level word B_2 can put on the DAC from a 7-bit panel value:
/// `(127 << 7) >> 2 = 0x0fe0`. The remaining 31 converter codes are not
/// reachable through the resonance parameter.
pub const PANEL_LEVEL_DAC_MAX: u16 = 0x0fe0;
pub const VCF_CALIBRATION_WORD: u16 = 6_272;
pub const VCF_CALIBRATION_HZ: f32 = 248.0;
pub const VCF_COUNTS_PER_OCTAVE: f32 = 1_142.0;
pub const PWM_DAC_MAX: u16 = 0x3fff;
pub const PWM_CV_AT_50_PERCENT_VOLTS: f32 = 6.0;
pub const PWM_CV_AT_95_PERCENT_VOLTS: f32 = 0.6;

/// Module-board noise path reconstructed from the Service Notes: the selected
/// 2SC945 avalanche source is AC-coupled into IC14 (BA662), whose output sees
/// 100 pF in parallel with 330 kOhm. The later 10 uF/47 kOhm coupling pole is
/// below 1 Hz and remains omitted from this first-order audio-band model.
pub const NOISE_HIGH_PASS_HZ: f32 = 1.0 / (2.0 * core::f32::consts::PI * 1.0e-6 * 4_700.0);
pub const NOISE_LOW_PASS_HZ: f32 = 1.0 / (2.0 * core::f32::consts::PI * 100.0e-12 * 330_000.0);
/// Numerical voltage-to-cell normalization. This is deliberately separate
/// from the schematic-derived resistor ratios so later captures can replace
/// it without changing the source topology.
pub const PROVISIONAL_VCF_INPUT_TRIM: f32 = 0.30;
/// The BA662 input/feedback cell works at a lower internal signal than the
/// externally observed VCF output. This scale restores that external level
/// after the four IR3109 integrators.
pub const VCF_OUTPUT_SCALE: f32 = 3.22;
/// Hardware noise-sweep crossover used by the input-bias compensation. Keeping
/// it in hertz makes the response independent of the host sample rate.
pub const VCF_INPUT_COMPENSATION_REFERENCE_HZ: f32 = 427.2;
/// Tiny deterministic stand-in for the unavoidable BA662/IR3109 thermal
/// excitation that starts high-resonance oscillation. Its absolute level is
/// intentionally below the audible signal floor pending calibrated captures.
pub const PROVISIONAL_VCF_THERMAL_EXCITATION: f32 = 1.0e-7;

#[derive(Clone, Copy, Debug)]
pub struct NoiseSource {
    seed: u32,
    low_pass_state: f32,
    dc_state: f32,
    low_pass_pole: f32,
    high_pass_pole: f32,
}

impl Default for NoiseSource {
    fn default() -> Self {
        Self::new()
    }
}

impl NoiseSource {
    const RESET_SEED: u32 = 0x6d2b_79f5;

    pub const fn new() -> Self {
        Self {
            seed: Self::RESET_SEED,
            low_pass_state: 0.0,
            dc_state: 0.0,
            low_pass_pole: 0.0,
            high_pass_pole: 0.0,
        }
    }

    pub fn prepare(&mut self, sample_rate: f32) {
        let rate = sample_rate.clamp(8_000.0, 192_000.0);
        self.low_pass_pole = libm::expf(-2.0 * core::f32::consts::PI * NOISE_LOW_PASS_HZ / rate);
        self.high_pass_pole = libm::expf(-2.0 * core::f32::consts::PI * NOISE_HIGH_PASS_HZ / rate);
        self.reset();
    }

    pub fn reset(&mut self) {
        self.seed = Self::RESET_SEED;
        self.low_pass_state = 0.0;
        self.dc_state = 0.0;
    }

    pub fn process(&mut self) -> f32 {
        // Four independent uniform draws approximate the broadband avalanche
        // carrier without importing the spectral embellishments of RF-106.
        let mut white = 0.0;
        for _ in 0..4 {
            let mut value = self.seed;
            value ^= value << 13;
            value ^= value >> 17;
            value ^= value << 5;
            self.seed = value;
            white += value as f32 / u32::MAX as f32 * 2.0 - 1.0;
        }
        self.shape(white * 0.5)
    }

    fn shape(&mut self, white: f32) -> f32 {
        self.low_pass_state =
            self.low_pass_pole * self.low_pass_state + (1.0 - self.low_pass_pole) * white;
        self.dc_state =
            self.high_pass_pole * self.dc_state + (1.0 - self.high_pass_pole) * self.low_pass_state;
        self.low_pass_state - self.dc_state
    }
}

/// Converts the firmware's internal 14-bit accumulator to the physical 12-bit
/// DAC bus described by the Service Notes (PB0..5 upper, PC2..7 lower).
pub const fn dac_word(internal_word: u16) -> u16 {
    if internal_word > ENVELOPE_MAX {
        DAC_MAX
    } else {
        internal_word >> 2
    }
}

/// Nominal WIDTH/FREQ-trim calibration from the RF-106 service procedure.
/// The public argument is the physical 12-bit DAC word. The documented 248 Hz
/// and 992 Hz anchors correspond to internal words 6272 and 8556; converting
/// back here preserves that established calibration while making quantization
/// at the real hardware boundary explicit.
pub fn nominal_vcf_hz(dac_word: u16) -> f32 {
    let internal_word = f32::from(dac_word.min(DAC_MAX)) * 4.0;
    let octaves = (internal_word - f32::from(VCF_CALIBRATION_WORD)) / VCF_COUNTS_PER_OCTAVE;
    VCF_CALIBRATION_HZ * libm::powf(2.0, octaves)
}

#[derive(Clone, Copy, Debug)]
pub struct A1qH80017a {
    filter_state: [f32; 4],
    vca_gain: f32,
    reset_seed: u32,
    thermal_seed: u32,
    vcf_sample_rate: f32,
    vcf_cutoff_dac: u16,
    vcf_resonance_dac: u16,
    vcf_coefficient: f32,
    vcf_feedback: f32,
    vcf_input_compensation: f32,
    vca_sample_rate: f32,
    vca_control: f32,
    vca_target: f32,
    vca_slew: f32,
}

impl A1qH80017a {
    const DEFAULT_SEED: u32 = 0x075b_cd15;

    pub const fn new() -> Self {
        Self::with_seed(Self::DEFAULT_SEED)
    }

    pub const fn with_seed(seed: u32) -> Self {
        let seed = if seed == 0 { Self::DEFAULT_SEED } else { seed };
        Self {
            filter_state: [0.0; 4],
            vca_gain: 0.0,
            reset_seed: seed,
            thermal_seed: seed,
            vcf_sample_rate: 0.0,
            vcf_cutoff_dac: u16::MAX,
            vcf_resonance_dac: u16::MAX,
            vcf_coefficient: 0.0,
            vcf_feedback: 0.0,
            vcf_input_compensation: 0.0,
            vca_sample_rate: 0.0,
            vca_control: -1.0,
            vca_target: 0.0,
            vca_slew: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.filter_state = [0.0; 4];
        self.vca_gain = 0.0;
        self.thermal_seed = self.reset_seed;
    }

    /// Four cascaded OTA-C stages with weak differential-pair saturation and
    /// global resonance feedback. The topology follows the 80017A/IR3109;
    /// resonance calibration remains provisional pending isolated captures.
    pub fn process_vcf(
        &mut self,
        input: f32,
        sample_rate: f32,
        cutoff_dac: u16,
        resonance_dac: u16,
    ) -> f32 {
        // B_2 holds the cutoff DAC word for an entire foreground control
        // cycle, while resonance and sample rate normally remain unchanged
        // for much longer. Rebuilding these transcendental coefficients per
        // audio sample wastes most of an ARM core without changing a single
        // output bit. Cache them against the exact hardware-domain inputs.
        if self.vcf_sample_rate.to_bits() != sample_rate.to_bits()
            || self.vcf_cutoff_dac != cutoff_dac
            || self.vcf_resonance_dac != resonance_dac
        {
            let cutoff = nominal_vcf_hz(cutoff_dac).clamp(5.0, sample_rate * 0.45);
            // Two half-steps keep the nonlinear feedback path stable through
            // the full control range without allocating an oversampling buffer.
            let sub_rate = sample_rate * 2.0;
            let g = libm::tanf(core::f32::consts::PI * cutoff / sub_rate);
            self.vcf_coefficient = (g / (1.0 + g)).clamp(0.0, 0.95);
            self.vcf_feedback = provisional_resonance_feedback(resonance_dac);
            self.vcf_input_compensation =
                provisional_vcf_input_compensation(self.vcf_feedback, cutoff);
            self.vcf_sample_rate = sample_rate;
            self.vcf_cutoff_dac = cutoff_dac;
            self.vcf_resonance_dac = resonance_dac;
        }
        let mut output = self.filter_state[3];
        for _ in 0..2 {
            let excitation = self.thermal_excitation();
            output = process_vcf_substep(
                &mut self.filter_state,
                (input + excitation) * self.vcf_input_compensation,
                self.vcf_coefficient,
                self.vcf_feedback,
            );
        }
        if output.is_finite() {
            output * VCF_OUTPUT_SCALE
        } else {
            self.filter_state = [0.0; 4];
            0.0
        }
    }

    /// Applies the per-voice BA662 control law plus a short control-port slew.
    /// The static law is derived from the documented PNP emitter converter;
    /// its absolute component tolerances remain a physical calibration item.
    pub fn process_vca(&mut self, input: f32, sample_rate: f32, control: f32) -> f32 {
        // The envelope and gate control are held by the emulated B_2 control
        // cycle. Preserve the per-sample BA662 slew, but only solve the static
        // transistor law when that held control (or the rate) actually changes.
        if self.vca_sample_rate.to_bits() != sample_rate.to_bits()
            || self.vca_control.to_bits() != control.to_bits()
        {
            self.vca_target = nominal_vca_gain(control);
            self.vca_slew = 1.0 - libm::expf(-1.0 / (sample_rate * 0.001).max(1.0));
            self.vca_sample_rate = sample_rate;
            self.vca_control = control;
        }
        self.vca_gain += self.vca_slew * (self.vca_target - self.vca_gain);
        input * self.vca_gain
    }

    pub const fn filter_state(&self) -> [f32; 4] {
        self.filter_state
    }

    pub fn vca_gain(&self) -> f32 {
        self.vca_gain
    }

    fn thermal_excitation(&mut self) -> f32 {
        self.thermal_seed = self
            .thermal_seed
            .wrapping_mul(196_314_165)
            .wrapping_add(907_633_515);
        let unit = self.thermal_seed as f32 / u32::MAX as f32;
        (unit * 2.0 - 1.0) * PROVISIONAL_VCF_THERMAL_EXCITATION
    }
}

/// Solves the instantaneous four-OTA feedback loop before committing the TPT
/// integrator states. Expressing the first OTA input as the scalar unknown
/// avoids a half-substep delay in the resonance path. Four Newton iterations
/// are ample because every differential-pair derivative is non-negative and
/// the loop Jacobian therefore remains at least one.
fn process_vcf_substep(states: &mut [f32; 4], input: f32, coefficient: f32, feedback: f32) -> f32 {
    let retained = *states;
    let mut first_input = input - feedback * retained[3];
    for _ in 0..4 {
        let (outputs, output_slope) = vcf_cascade(first_input, retained, coefficient);
        let residual = first_input + feedback * outputs[3] - input;
        first_input -= residual / (1.0 + feedback * output_slope);
    }

    let (outputs, _) = vcf_cascade(first_input, retained, coefficient);
    for index in 0..4 {
        // In a trapezoidal integrator the retained state is the reflection of
        // the previous state through the current stage output.
        states[index] = 2.0 * outputs[index] - retained[index];
    }
    outputs[3]
}

fn vcf_cascade(first_input: f32, states: [f32; 4], coefficient: f32) -> ([f32; 4], f32) {
    let mut outputs = [0.0; 4];
    let mut stage_input = first_input;
    let mut output_slope = 1.0;
    for index in 0..4 {
        let (driven, drive_slope) = ota_saturate_with_slope(stage_input);
        outputs[index] = states[index] + coefficient * (driven - states[index]);
        output_slope *= coefficient * drive_slope;
        stage_input = outputs[index];
    }
    (outputs, output_slope)
}

impl Default for A1qH80017a {
    fn default() -> Self {
        Self::new()
    }
}

/// Compact Padé approximation to a differential pair. Its bounded output is
/// used only inside the analog model; digital control paths remain bit exact.
pub fn ota_saturate(value: f32) -> f32 {
    ota_saturate_with_slope(value).0
}

fn ota_saturate_with_slope(value: f32) -> (f32, f32) {
    let unclamped = value;
    let value = value.clamp(-3.0, 3.0);
    let square = value * value;
    let denominator = 27.0 + 9.0 * square;
    let output = value * (27.0 + square) / denominator;
    let slope = if unclamped == value {
        9.0 * (square - 9.0) * (square - 9.0) / (denominator * denominator)
    } else {
        0.0
    };
    (output, slope)
}

/// Evidence-bounded JUNO-106 resonance-CV transfer. The polynomial is the
/// retained fit to hardware noise-grid peak measurements; the original raw
/// captures are not in this repository, so it remains provisional rather than
/// being promoted to a unit-independent component law.
pub fn provisional_resonance_feedback(resonance_dac: u16) -> f32 {
    let r = f32::from(resonance_dac.min(PANEL_LEVEL_DAC_MAX)) / f32::from(PANEL_LEVEL_DAC_MAX);
    let r2 = r * r;
    let r3 = r2 * r;
    let r4 = r2 * r2;
    1.24 * (4.7116 * r - 6.5743 * r2 + 13.4633 * r3 - 8.2197 * r4)
}

/// Models the BA662 input summing path whose signal and resonance-return arms
/// do not have the same transconductance. The linear-in-feedback term is the
/// measured Q compensation; the shallow frequency term is the retained
/// hardware noise-sweep fit and is bounded outside its measured useful range.
pub fn provisional_vcf_input_compensation(feedback: f32, cutoff_hz: f32) -> f32 {
    let q_compensation = 0.379 + 0.087 * feedback;
    let frequency_gain = libm::powf(
        (cutoff_hz.max(1.0) / VCF_INPUT_COMPENSATION_REFERENCE_HZ).max(1.0e-6),
        -0.10,
    )
    .clamp(0.65, 1.2);
    q_compensation * frequency_gain
}

/// Panel resonance passes through the same raw<<7 internal word and 12-bit DAC
/// truncation as the VCF cutoff base. The analog feedback law remains open.
pub fn resonance_dac(resonance: f32) -> u16 {
    dac_word(level_dac(resonance))
}

/// Circuit-derived nominal gain for TR17 (2SA1015), R95=22K and the BA662
/// control input. Newton iteration solves I*R + Vt*ln(I/Is) = 10*control.
pub fn nominal_vca_gain(control: f32) -> f32 {
    const RESISTANCE: f32 = 22_000.0;
    const THERMAL_VOLTAGE: f32 = 0.026;
    const SATURATION_CURRENT: f32 = 1.0e-14;
    let voltage = control.clamp(0.0, 1.0) * 10.0;
    if voltage <= 0.0 {
        return 0.0;
    }
    let current_at = |applied: f32| {
        let mut current = ((applied - 0.6) / RESISTANCE).max(SATURATION_CURRENT);
        for _ in 0..6 {
            let residual = current * RESISTANCE
                + THERMAL_VOLTAGE * libm::logf(current / SATURATION_CURRENT)
                - applied;
            let derivative = RESISTANCE + THERMAL_VOLTAGE / current;
            current = (current - residual / derivative).max(SATURATION_CURRENT);
        }
        current
    };
    (current_at(voltage) / current_at(10.0)).clamp(0.0, 1.0)
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EnvelopeStage {
    #[default]
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Envelope {
    value: u16,
    stage: EnvelopeStage,
    samples_until_control_cycle: u32,
}

impl Envelope {
    pub const fn new() -> Self {
        Self {
            value: 0,
            stage: EnvelopeStage::Idle,
            samples_until_control_cycle: 0,
        }
    }

    pub fn note_on(&mut self) {
        // B_2 does not clear the envelope accumulator when a running voice is
        // retriggered. Attack resumes from the retained 14-bit DAC value.
        self.stage = EnvelopeStage::Attack;
    }

    pub fn note_off(&mut self) {
        if self.stage != EnvelopeStage::Idle {
            self.stage = EnvelopeStage::Release;
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub const fn stage(&self) -> EnvelopeStage {
        self.stage
    }

    pub const fn value(&self) -> u16 {
        self.value
    }

    pub fn normalized(&self) -> f32 {
        f32::from(self.value) / f32::from(ENVELOPE_MAX)
    }

    pub fn process(
        &mut self,
        sample_rate: f32,
        attack: f32,
        decay: f32,
        sustain: f32,
        release: f32,
    ) -> f32 {
        if self.samples_until_control_cycle == 0 {
            self.tick(
                panel_raw(attack),
                panel_raw(decay),
                panel_raw(sustain),
                panel_raw(release),
            );
            self.samples_until_control_cycle =
                libm::roundf(sample_rate * CONTROL_CYCLE_SECONDS).max(1.0) as u32;
        }
        self.samples_until_control_cycle -= 1;
        self.normalized()
    }

    fn tick(&mut self, attack: u8, decay: u8, sustain: u8, release: u8) {
        let sustain = u16::from(sustain) << 7;
        match self.stage {
            EnvelopeStage::Idle => self.value = 0,
            EnvelopeStage::Attack => {
                let next = u32::from(self.value) + u32::from(attack_increment(attack));
                if next > u32::from(ENVELOPE_MAX) {
                    self.value = ENVELOPE_MAX;
                    self.stage = EnvelopeStage::Decay;
                } else {
                    self.value = next as u16;
                }
            }
            EnvelopeStage::Decay => {
                if self.value <= sustain {
                    self.value = sustain;
                    self.stage = EnvelopeStage::Sustain;
                } else {
                    let remainder =
                        calc_decay(self.value - sustain, decay_release_coefficient(decay));
                    self.value = sustain + remainder;
                    if remainder == 0 {
                        self.stage = EnvelopeStage::Sustain;
                    }
                }
            }
            EnvelopeStage::Sustain => self.value = sustain,
            EnvelopeStage::Release => {
                self.value = calc_decay(self.value, decay_release_coefficient(release));
                if self.value == 0 {
                    self.stage = EnvelopeStage::Idle;
                }
            }
        }
    }
}

pub fn panel_raw(value: f32) -> u8 {
    libm::roundf(value.clamp(0.0, 1.0) * 127.0) as u8
}

/// Clean-room reconstruction of the B_2 attack timing law. The first half is
/// harmonic; the remaining shallow regions are compact linear fits. Sparse
/// executed checkpoints constrain the regions and endpoints without embedding
/// the firmware lookup table.
pub fn attack_increment(raw: u8) -> u16 {
    let raw = raw.min(127);
    if raw == 0 {
        return 0x4000;
    }
    if raw <= 63 {
        return libm::roundf(8192.0 / f32::from(raw)) as u16;
    }
    let normalized = f32::from(raw) / 127.0;
    if raw <= 86 {
        (libm::roundf(305.03 - 352.26 * normalized) as u16).min(127)
    } else if raw <= 107 {
        libm::roundf(194.74 - 190.50 * normalized) as u16
    } else if raw <= 121 {
        libm::roundf(86.37 - 62.52 * normalized) as u16
    } else {
        libm::roundf(148.0 - 127.0 * normalized).max(1.0) as u16
    }
}

/// Generates the observed retained-fraction law from seven linear regions.
/// This is behavioral code, not a copied 128-word firmware table.
pub fn decay_release_coefficient(raw: u8) -> u16 {
    const COUNTS: [u8; 7] = [4, 1, 10, 28, 22, 58, 4];
    const STEPS: [u16; 7] = [0x2000, 0x1000, 0x0800, 0x0080, 0x000c, 0x0004, 0x0001];
    let mut value = 0x1000_u16;
    let mut remaining = raw.min(127);
    let mut segment = 0;
    while segment < COUNTS.len() && remaining != 0 {
        let used = if remaining < COUNTS[segment] {
            remaining
        } else {
            COUNTS[segment]
        };
        value = value.wrapping_add(STEPS[segment].wrapping_mul(u16::from(used)));
        remaining -= used;
        segment += 1;
    }
    value
}

/// Reproduces the three partial 8x8 products used by B_2. The low-byte by
/// low-byte term is intentionally absent, matching the executed D7811G code.
pub fn calc_decay(value: u16, coefficient: u16) -> u16 {
    let vh = value >> 8;
    let vl = value & 0xff;
    let ch = coefficient >> 8;
    let cl = coefficient & 0xff;
    vh * ch + ((vh * cl) >> 8) + ((vl * ch) >> 8)
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum VcaMode {
    #[default]
    Envelope,
    Gate,
}

pub fn vca_control(mode: VcaMode, envelope: f32, running: bool) -> f32 {
    match mode {
        VcaMode::Envelope => envelope,
        VcaMode::Gate => {
            if running {
                1.0
            } else {
                0.0
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum VcfEnvelopePolarity {
    #[default]
    Positive,
    Negative,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VcfControlWord {
    /// Firmware accumulator before the physical DAC boundary.
    pub value: u16,
    /// Word actually held by the per-voice VCF sample-and-hold.
    pub dac: u16,
    pub clipped_low: bool,
    pub clipped_high: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct VcfControl {
    held: VcfControlWord,
    samples_until_control_cycle: u32,
}

impl VcfControl {
    pub const fn new() -> Self {
        Self {
            held: VcfControlWord {
                value: 0,
                dac: 0,
                clipped_low: false,
                clipped_high: false,
            },
            samples_until_control_cycle: 0,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    #[allow(clippy::too_many_arguments)]
    pub fn process(
        &mut self,
        sample_rate: f32,
        cutoff: f32,
        envelope: u16,
        envelope_amount: f32,
        envelope_polarity: VcfEnvelopePolarity,
        key_track: f32,
        note_8_8: u16,
        lfo_offset: i16,
        bend_offset: i16,
    ) -> VcfControlWord {
        if self.samples_until_control_cycle == 0 {
            self.held = vcf_control_word(
                cutoff,
                envelope,
                envelope_amount,
                envelope_polarity,
                key_track,
                note_8_8,
                lfo_offset,
                bend_offset,
            );
            self.samples_until_control_cycle =
                libm::roundf(sample_rate * CONTROL_CYCLE_SECONDS).max(1.0) as u32;
        }
        self.samples_until_control_cycle -= 1;
        self.held
    }
}

/// Recreates the B_2 per-voice VCF DAC arithmetic in its native 14-bit domain.
/// `note_8_8` is the same held pitch word sent to the DCO path. LFO and bend are
/// signed 14-bit destination offsets already computed by their control blocks.
#[allow(clippy::too_many_arguments)]
pub fn vcf_control_word(
    cutoff: f32,
    envelope: u16,
    envelope_amount: f32,
    envelope_polarity: VcfEnvelopePolarity,
    key_track: f32,
    note_8_8: u16,
    lfo_offset: i16,
    bend_offset: i16,
) -> VcfControlWord {
    let mut accumulator = level_dac(cutoff);
    let mut underflow = false;
    accumulator = vcf_apply_signed(accumulator, lfo_offset, &mut underflow);
    accumulator = vcf_apply_signed(accumulator, bend_offset, &mut underflow);
    let envelope_scaled = scale_16_by_8(envelope, doubled_7bit(envelope_amount));
    accumulator = match envelope_polarity {
        VcfEnvelopePolarity::Positive => vcf_add(accumulator, envelope_scaled, &mut underflow),
        VcfEnvelopePolarity::Negative => vcf_sub(accumulator, envelope_scaled, &mut underflow),
    };

    // Firmware derives 3/8 of pitch, centers it at middle C (60 * 256),
    // then applies the doubled seven-bit keyboard-follow amount.
    let pitch_three_eighths = (u32::from(note_8_8) >> 2) + (u32::from(note_8_8) >> 3);
    let middle_c_three_eighths = (60_u32 * 256 * 3) / 8;
    let key_scale = doubled_7bit(key_track);
    accumulator = if pitch_three_eighths < middle_c_three_eighths {
        let distance = (middle_c_three_eighths - pitch_three_eighths) as u16;
        vcf_sub(
            accumulator,
            scale_16_by_8(distance, key_scale),
            &mut underflow,
        )
    } else {
        let distance = (pitch_three_eighths - middle_c_three_eighths) as u16;
        vcf_add(
            accumulator,
            scale_16_by_8(distance, key_scale),
            &mut underflow,
        )
    };
    let clipped_low = accumulator > ENVELOPE_MAX && underflow;
    let clipped_high = accumulator > ENVELOPE_MAX && !underflow;
    let value = if clipped_low {
        0
    } else if clipped_high {
        ENVELOPE_MAX
    } else {
        accumulator
    };
    VcfControlWord {
        value,
        dac: dac_word(value),
        clipped_low,
        clipped_high,
    }
}

fn vcf_apply_signed(accumulator: u16, offset: i16, underflow: &mut bool) -> u16 {
    if offset < 0 {
        vcf_sub(accumulator, offset.unsigned_abs(), underflow)
    } else {
        vcf_add(accumulator, offset as u16, underflow)
    }
}

fn vcf_add(accumulator: u16, value: u16, underflow: &mut bool) -> u16 {
    let (result, carried) = accumulator.overflowing_add(value);
    if carried {
        *underflow = false;
    }
    result
}

fn vcf_sub(accumulator: u16, value: u16, underflow: &mut bool) -> u16 {
    let (result, borrowed) = accumulator.overflowing_sub(value);
    if borrowed {
        *underflow = true;
    }
    result
}

/// The two MUL instructions and byte accumulation used throughout B_2 when a
/// 16-bit control word is scaled by a doubled 7-bit panel value.
pub fn scale_16_by_8(value: u16, scale: u8) -> u16 {
    let high = (value >> 8) as u8;
    let low = value as u8;
    u16::from(high) * u16::from(scale) + ((u16::from(low) * u16::from(scale)) >> 8)
}

/// The external clock selected by the PF6/PF7 range divider.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DcoRange {
    SixteenFoot,
    #[default]
    EightFoot,
    FourFoot,
}

impl DcoRange {
    pub const fn clock_hz(self) -> u32 {
        match self {
            Self::SixteenFoot => 1_000_000,
            Self::EightFoot => 2_000_000,
            Self::FourFoot => 4_000_000,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PitProgram {
    pub control_word: u8,
    pub low_byte: u8,
    pub high_byte: u8,
    pub divider: u16,
}

/// Recreates the observable B_2 pitch destination without carrying its ROM
/// table. The complete clean-room 24..=108 range is indexed directly and
/// adjacent values are linearly interpolated with the pitch fraction, as B_2
/// does.
pub fn pit_program(note_8_8: u16) -> PitProgram {
    const OBSERVED_COUNTS: [u16; 85] = [
        61_280, 57_834, 54_575, 51_509, 48_612, 45_882, 43_297, 40_862, 38_565, 36_396, 34_350,
        32_421, 30_601, 28_882, 27_255, 25_725, 24_280, 22_916, 21_626, 20_411, 19_264, 18_180,
        17_160, 16_198, 15_289, 14_431, 13_618, 12_854, 12_132, 11_451, 10_807, 10_200, 9_627,
        9_085, 8_576, 8_096, 7_642, 7_213, 6_806, 6_424, 6_064, 5_723, 5_402, 5_099, 4_812, 4_541,
        4_287, 4_048, 3_821, 3_606, 3_402, 3_211, 3_031, 2_861, 2_701, 2_549, 2_406, 2_270, 2_143,
        2_024, 1_910, 1_803, 1_701, 1_605, 1_516, 1_430, 1_351, 1_275, 1_203, 1_135, 1_072, 1_012,
        955, 902, 851, 803, 758, 715, 676, 638, 602, 568, 536, 506, 477,
    ];
    let note = f32::from(note_8_8) / 256.0;
    let bounded = note.clamp(24.0, 108.0);
    let integer = libm::floorf(bounded) as usize;
    let fraction = bounded - integer as f32;
    let index = integer - 24;
    let lower = f32::from(OBSERVED_COUNTS[index]);
    let upper = f32::from(OBSERVED_COUNTS[(index + 1).min(84)]);
    let divider = libm::roundf(lower + (upper - lower) * fraction).clamp(1.0, 65_535.0) as u16;
    PitProgram {
        control_word: 0x36,
        low_byte: divider as u8,
        high_byte: (divider >> 8) as u8,
        divider,
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DcoFrame {
    pub saw: f32,
    pub pulse: f32,
    pub sub: f32,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PwmMode {
    Lfo,
    #[default]
    Manual,
}

#[derive(Clone, Copy, Debug)]
pub struct SourceControl {
    pub saw_enabled: bool,
    pub pulse_enabled: bool,
    pub sub_level_dac: u16,
    pub noise_level_dac: u16,
    pub pwm_depth: u8,
    pub pwm_mode: PwmMode,
}

impl SourceControl {
    pub fn from_native(
        saw_enabled: bool,
        pulse_enabled: bool,
        pwm_depth: f32,
        noise_level: f32,
        sub_level: f32,
        pwm_mode: PwmMode,
    ) -> Self {
        Self {
            saw_enabled,
            pulse_enabled,
            sub_level_dac: level_dac(sub_level),
            noise_level_dac: level_dac(noise_level),
            pwm_depth: doubled_7bit(pwm_depth),
            pwm_mode,
        }
    }

    pub fn pwm_dac(self, lfo_source: u16) -> u16 {
        let source = match self.pwm_mode {
            PwmMode::Manual => PWM_DAC_MAX,
            PwmMode::Lfo => lfo_source.min(PWM_DAC_MAX),
        };
        let product = u32::from(source) * u32::from(self.pwm_depth);
        PWM_DAC_MAX.saturating_sub((product >> 8) as u16)
    }

    /// Nominal PWM control voltage at the MC5534 comparator. Service Notes p.9
    /// anchors the retained DAC range at +6 V and +0.6 V.
    pub fn nominal_pwm_cv(self, lfo_source: u16) -> f32 {
        let normalized = f32::from(self.pwm_dac(lfo_source)) / f32::from(PWM_DAC_MAX);
        PWM_CV_AT_95_PERCENT_VOLTS
            + normalized * (PWM_CV_AT_50_PERCENT_VOLTS - PWM_CV_AT_95_PERCENT_VOLTS)
    }

    /// Documented nominal comparator transfer: +6 V produces 50% duty and
    /// +0.6 V produces 95%. The saw ramp makes the interpolation linear in the
    /// ideal model; physical MC5534 curvature remains a measurement input.
    pub fn pulse_duty(self, lfo_source: u16) -> f32 {
        let voltage_span = PWM_CV_AT_50_PERCENT_VOLTS - PWM_CV_AT_95_PERCENT_VOLTS;
        0.95 - 0.45
            * ((self.nominal_pwm_cv(lfo_source) - PWM_CV_AT_95_PERCENT_VOLTS) / voltage_span)
    }

    pub fn mix(self, frame: DcoFrame, noise: f32) -> f32 {
        // The MC5534 exposes one WAVE node for saw/pulse. On the Module Board
        // WAVE, SUB and NOISE reach the per-voice VCF through 33k, 27k and 39k
        // paths respectively. Normalize conductance to the WAVE path and leave
        // one explicit input trim for later voltage captures.
        const WAVE_RESISTANCE: f32 = 33_000.0;
        const SUB_RESISTANCE: f32 = 27_000.0;
        const NOISE_RESISTANCE: f32 = 39_000.0;
        let mut wave = 0.0;
        if self.saw_enabled {
            wave += frame.saw;
        }
        if self.pulse_enabled {
            wave += frame.pulse;
        }
        let mut mixed = wave;
        // The hardware has no SUB on/off switch: the SUB level slider and its
        // DAC value are the complete source control. A zero level is the only
        // way to silence this path.
        mixed += frame.sub * self.sub_gain() * (WAVE_RESISTANCE / SUB_RESISTANCE);
        mixed += noise * self.noise_gain() * (WAVE_RESISTANCE / NOISE_RESISTANCE);
        mixed * PROVISIONAL_VCF_INPUT_TRIM
    }

    pub fn sub_gain(self) -> f32 {
        f32::from(self.sub_level_dac) / 16_256.0
    }

    pub fn noise_gain(self) -> f32 {
        f32::from(self.noise_level_dac) / 16_256.0
    }
}

pub fn doubled_7bit(value: f32) -> u8 {
    (libm::roundf(value.clamp(0.0, 1.0) * 127.0) as u8).saturating_mul(2)
}

pub fn level_dac(value: f32) -> u16 {
    u16::from(libm::roundf(value.clamp(0.0, 1.0) * 127.0) as u8) << 7
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Dco {
    phase: f32,
    sub_high: bool,
    held_divider: u16,
    held_pulse_width: f32,
    samples_until_control_cycle: u32,
}

impl Dco {
    pub const fn new() -> Self {
        Self {
            phase: 0.0,
            sub_high: false,
            held_divider: 0,
            held_pulse_width: 0.5,
            samples_until_control_cycle: 0,
        }
    }

    pub fn reset(&mut self) {
        self.phase = 0.0;
        self.sub_high = false;
        self.samples_until_control_cycle = 0;
    }

    pub fn phase(&self) -> f32 {
        self.phase
    }

    pub fn held_divider(&self) -> u16 {
        self.held_divider
    }

    pub fn held_pulse_width(&self) -> f32 {
        self.held_pulse_width
    }

    pub fn process(
        &mut self,
        sample_rate: f32,
        note_8_8: u16,
        range: DcoRange,
        pulse_width: f32,
    ) -> DcoFrame {
        if self.samples_until_control_cycle == 0 {
            self.held_divider = pit_program(note_8_8).divider;
            self.held_pulse_width = pulse_width.clamp(0.03, 0.97);
            self.samples_until_control_cycle =
                libm::roundf(sample_rate * CONTROL_CYCLE_SECONDS).max(1.0) as u32;
        }
        self.samples_until_control_cycle -= 1;
        let frequency = range.clock_hz() as f32 / f32::from(self.held_divider);
        let increment = (frequency / sample_rate).clamp(0.0, 0.49);
        self.phase += increment;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
            self.sub_high = !self.sub_high;
        }
        let width = self.held_pulse_width;
        let mut saw = self.phase * 2.0 - 1.0;
        saw -= poly_blep(self.phase, increment);
        let mut pulse = if self.phase < width { 1.0 } else { -1.0 };
        pulse += poly_blep(self.phase, increment);
        pulse -= poly_blep(wrap(self.phase + 1.0 - width), increment);
        // The hardware sub oscillator is the DCO square divided by two.  Keep
        // that exact divider phase, but describe it as a continuous phase so
        // both discontinuities can receive the same PolyBLEP treatment as the
        // primary pulse wave.
        let sub_phase = if self.sub_high {
            self.phase * 0.5
        } else {
            0.5 + self.phase * 0.5
        };
        let sub_increment = increment * 0.5;
        let mut sub = if sub_phase < 0.5 { 1.0 } else { -1.0 };
        sub += poly_blep(sub_phase, sub_increment);
        sub -= poly_blep(wrap(sub_phase + 0.5), sub_increment);
        DcoFrame { saw, pulse, sub }
    }
}

fn wrap(value: f32) -> f32 {
    value - libm::floorf(value)
}

fn poly_blep(phase: f32, increment: f32) -> f32 {
    if increment <= 0.0 {
        return 0.0;
    }
    if phase < increment {
        let x = phase / increment;
        x + x - x * x - 1.0
    } else if phase > 1.0 - increment {
        let x = (phase - 1.0) / increment;
        x * x + x + x + 1.0
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_clocks_transpose_the_same_counter_program() {
        let program = pit_program(60 << 8);
        let sixteen = DcoRange::SixteenFoot.clock_hz() as f32 / f32::from(program.divider);
        let eight = DcoRange::EightFoot.clock_hz() as f32 / f32::from(program.divider);
        let four = DcoRange::FourFoot.clock_hz() as f32 / f32::from(program.divider);
        assert_eq!(sixteen * 2.0, eight);
        assert_eq!(eight * 2.0, four);
    }

    #[test]
    fn sub_oscillator_toggles_once_per_dco_cycle() {
        let mut dco = Dco::new();
        let mut transitions = 0;
        let mut last_positive = dco.process(48_000.0, 69 << 8, DcoRange::EightFoot, 0.5).sub >= 0.0;
        for _ in 0..48_000 {
            let next = dco.process(48_000.0, 69 << 8, DcoRange::EightFoot, 0.5).sub;
            let next_positive = next >= 0.0;
            if next_positive != last_positive {
                transitions += 1;
                last_positive = next_positive;
            }
        }
        assert!((438..=442).contains(&transitions));
    }

    #[test]
    fn sub_oscillator_bandlimits_both_divider_edges() {
        let mut dco = Dco::new();
        let mut positive_edge_samples = 0;
        let mut negative_edge_samples = 0;
        for _ in 0..48_000 {
            let sub = dco.process(48_000.0, 96 << 8, DcoRange::FourFoot, 0.5).sub;
            if sub > 0.0 && sub < 0.999 {
                positive_edge_samples += 1;
            } else if sub < 0.0 && sub > -0.999 {
                negative_edge_samples += 1;
            }
        }
        assert!(positive_edge_samples > 100);
        assert!(negative_edge_samples > 100);
    }

    #[test]
    fn pwm_comparator_follows_service_note_voltage_and_duty_anchors() {
        let open = SourceControl::from_native(false, true, 0.0, 0.0, 0.0, PwmMode::Manual);
        assert_eq!(open.pwm_dac(0), PWM_DAC_MAX);
        assert!((open.nominal_pwm_cv(0) - 6.0).abs() < f32::EPSILON);
        assert!((open.pulse_duty(0) - 0.5).abs() < f32::EPSILON);

        let narrow = SourceControl::from_native(false, true, 1.0, 0.0, 0.0, PwmMode::Manual);
        assert_eq!(narrow.pwm_dac(0), 128);
        assert!((narrow.nominal_pwm_cv(0) - 0.642_19).abs() < 1.0e-5);
        assert!((narrow.pulse_duty(0) - 0.946_484).abs() < 1.0e-5);
    }

    #[test]
    fn lfo_pwm_traverses_the_same_documented_comparator_polarity() {
        let control = SourceControl::from_native(false, true, 1.0, 0.0, 0.0, PwmMode::Lfo);
        assert_eq!(control.pwm_dac(1), PWM_DAC_MAX);
        assert_eq!(control.pwm_dac(PWM_DAC_MAX), 128);
        assert!(control.pulse_duty(1) < control.pulse_duty(0x2000));
        assert!(control.pulse_duty(0x2000) < control.pulse_duty(PWM_DAC_MAX));
        assert!((control.pulse_duty(1) - 0.5).abs() < 1.0e-4);
        assert!((control.pulse_duty(PWM_DAC_MAX) - 0.946_484).abs() < 1.0e-5);
    }

    #[test]
    fn disabled_primary_sources_do_not_invent_a_fallback_saw() {
        let control = SourceControl::from_native(false, false, 0.0, 0.0, 0.0, PwmMode::Manual);
        assert_eq!(
            control.mix(
                DcoFrame {
                    saw: 1.0,
                    pulse: 1.0,
                    sub: 1.0
                },
                1.0
            ),
            0.0
        );
    }

    #[test]
    fn sub_level_is_the_only_sub_oscillator_control() {
        let audible = SourceControl::from_native(false, false, 0.0, 0.0, 1.0, PwmMode::Manual);
        let silent = SourceControl::from_native(false, false, 0.0, 0.0, 0.0, PwmMode::Manual);
        let sub_only = DcoFrame {
            saw: 0.0,
            pulse: 0.0,
            sub: 1.0,
        };
        assert!(audible.mix(sub_only, 0.0) > 0.0);
        assert_eq!(silent.mix(sub_only, 0.0), 0.0);
    }

    #[test]
    fn source_mixer_uses_module_board_resistor_ratios() {
        let full = SourceControl::from_native(true, false, 0.0, 1.0, 1.0, PwmMode::Manual);
        let wave_only = SourceControl::from_native(true, false, 0.0, 0.0, 0.0, PwmMode::Manual);
        let wave = wave_only.mix(
            DcoFrame {
                saw: 1.0,
                pulse: 0.0,
                sub: 0.0,
            },
            0.0,
        );
        let sub = full.mix(
            DcoFrame {
                saw: 0.0,
                pulse: 0.0,
                sub: 1.0,
            },
            0.0,
        );
        let noise = full.mix(DcoFrame::default(), 1.0);
        assert!((sub / wave - 33.0 / 27.0).abs() < 1.0e-5);
        assert!((noise / wave - 33.0 / 39.0).abs() < 1.0e-5);
    }

    #[test]
    fn module_noise_poles_come_from_documented_components() {
        assert!((NOISE_HIGH_PASS_HZ - 33.86275).abs() < 0.001);
        assert!((NOISE_LOW_PASS_HZ - 4_822.877).abs() < 0.01);
        let mut source = NoiseSource::new();
        source.prepare(48_000.0);
        let mut energy = 0.0;
        for _ in 0..48_000 {
            let sample = source.process();
            assert!(sample.is_finite());
            energy += sample * sample;
        }
        assert!(energy > 1.0);
    }

    #[test]
    fn noise_reset_is_deterministic_for_reproducible_renders() {
        let mut source = NoiseSource::new();
        source.prepare(48_000.0);
        let first = source.process();
        for _ in 0..100 {
            source.process();
        }
        source.reset();
        assert_eq!(source.process(), first);
    }

    #[test]
    fn divider_and_pwm_are_held_for_one_executed_control_cycle() {
        let mut dco = Dco::new();
        let samples = libm::roundf(48_000.0 * CONTROL_CYCLE_SECONDS) as usize;
        dco.process(48_000.0, 48 << 8, DcoRange::EightFoot, 0.25);
        assert_eq!(dco.held_divider(), 15_289);
        assert_eq!(dco.held_pulse_width(), 0.25);
        for _ in 1..samples {
            dco.process(48_000.0, 60 << 8, DcoRange::EightFoot, 0.75);
            assert_eq!(dco.held_divider(), 15_289);
            assert_eq!(dco.held_pulse_width(), 0.25);
        }
        dco.process(48_000.0, 60 << 8, DcoRange::EightFoot, 0.75);
        assert_eq!(dco.held_divider(), 7_642);
        assert_eq!(dco.held_pulse_width(), 0.75);
    }

    #[test]
    fn discrete_envelope_retriggers_from_retained_level() {
        let mut envelope = Envelope::new();
        envelope.note_on();
        for _ in 0..8 {
            envelope.tick(32, 64, 64, 64);
        }
        let before_release = envelope.value();
        envelope.note_off();
        envelope.tick(32, 64, 64, 64);
        let during_release = envelope.value();
        assert!(during_release < before_release);
        envelope.note_on();
        envelope.tick(32, 64, 64, 64);
        assert_eq!(envelope.value(), during_release + attack_increment(32));
    }

    #[test]
    fn gate_vca_uses_running_state_instead_of_release_envelope() {
        assert_eq!(vca_control(VcaMode::Gate, 0.75, true), 1.0);
        assert_eq!(vca_control(VcaMode::Gate, 0.75, false), 0.0);
        assert_eq!(vca_control(VcaMode::Envelope, 0.75, false), 0.75);
    }

    #[test]
    fn vcf_word_is_centered_at_middle_c_and_tracks_both_directions() {
        let center = vcf_control_word(
            0.5,
            0,
            0.0,
            VcfEnvelopePolarity::Positive,
            1.0,
            60 << 8,
            0,
            0,
        );
        let low = vcf_control_word(
            0.5,
            0,
            0.0,
            VcfEnvelopePolarity::Positive,
            1.0,
            48 << 8,
            0,
            0,
        );
        let high = vcf_control_word(
            0.5,
            0,
            0.0,
            VcfEnvelopePolarity::Positive,
            1.0,
            72 << 8,
            0,
            0,
        );
        assert_eq!(center.value, level_dac(0.5));
        assert!(low.value < center.value);
        assert!(high.value > center.value);
    }

    #[test]
    fn vcf_envelope_polarity_and_offsets_saturate_in_dac_domain() {
        let positive = vcf_control_word(
            0.5,
            ENVELOPE_MAX,
            1.0,
            VcfEnvelopePolarity::Positive,
            0.0,
            60 << 8,
            0,
            0,
        );
        let negative = vcf_control_word(
            0.5,
            ENVELOPE_MAX,
            1.0,
            VcfEnvelopePolarity::Negative,
            0.0,
            60 << 8,
            -4095,
            0,
        );
        assert_eq!(positive.value, ENVELOPE_MAX);
        assert_eq!(positive.dac, DAC_MAX);
        assert!(positive.clipped_high);
        assert_eq!(negative.value, 0);
        assert_eq!(negative.dac, 0);
        assert!(negative.clipped_low);
    }

    #[test]
    fn vcf_sequential_borrow_is_cancelled_only_by_a_later_carry() {
        let recovered = vcf_control_word(
            0.0,
            0,
            0.0,
            VcfEnvelopePolarity::Positive,
            0.0,
            60 << 8,
            -0x0fcf,
            0x0fe0,
        );
        assert_eq!(recovered.value, 0x0011);
        assert_eq!(recovered.dac, 0x004);
        assert!(!recovered.clipped_low && !recovered.clipped_high);

        let underflow = vcf_control_word(
            0.0,
            0,
            0.0,
            VcfEnvelopePolarity::Positive,
            0.0,
            60 << 8,
            0x0fcf,
            -0x0fe0,
        );
        assert_eq!(underflow.value, 0);
        assert_eq!(underflow.dac, 0);
        assert!(underflow.clipped_low);
    }

    #[test]
    fn vcf_full_scale_bend_enters_the_accumulator_without_rescaling() {
        let positive = vcf_control_word(
            0.5,
            0,
            0.0,
            VcfEnvelopePolarity::Positive,
            0.0,
            60 << 8,
            0,
            0x0fe0,
        );
        let negative = vcf_control_word(
            0.5,
            0,
            0.0,
            VcfEnvelopePolarity::Positive,
            0.0,
            60 << 8,
            0,
            -0x0fe0,
        );
        assert_eq!(positive.dac, 0xbf8);
        assert_eq!(negative.dac, 0x408);
    }

    #[test]
    fn vcf_word_is_held_for_one_executed_control_cycle() {
        let mut control = VcfControl::new();
        let samples = libm::roundf(48_000.0 * CONTROL_CYCLE_SECONDS) as usize;
        let first = control.process(
            48_000.0,
            0.25,
            0,
            0.0,
            VcfEnvelopePolarity::Positive,
            0.0,
            60 << 8,
            0,
            0,
        );
        for _ in 1..samples {
            assert_eq!(
                control.process(
                    48_000.0,
                    0.75,
                    ENVELOPE_MAX,
                    1.0,
                    VcfEnvelopePolarity::Positive,
                    1.0,
                    72 << 8,
                    0,
                    0,
                ),
                first
            );
        }
        assert_ne!(
            control.process(
                48_000.0,
                0.75,
                ENVELOPE_MAX,
                1.0,
                VcfEnvelopePolarity::Positive,
                1.0,
                72 << 8,
                0,
                0,
            ),
            first
        );
    }

    #[test]
    fn nominal_vcf_curve_matches_service_calibration_anchors() {
        assert!((nominal_vcf_hz(dac_word(VCF_CALIBRATION_WORD)) - 248.0).abs() < 0.001);
        assert!((nominal_vcf_hz(dac_word(VCF_CALIBRATION_WORD + 2_284)) - 992.0).abs() < 0.01);
        let octave_dac = dac_word(VCF_CALIBRATION_WORD + 1_142);
        let octave = nominal_vcf_hz(octave_dac);
        // The physical DAC truncates this ideal internal anchor by two counts.
        let expected = VCF_CALIBRATION_HZ
            * libm::powf(
                2.0,
                (f32::from(octave_dac) * 4.0 - f32::from(VCF_CALIBRATION_WORD))
                    / VCF_COUNTS_PER_OCTAVE,
            );
        assert!((octave - expected).abs() < 0.001);
        assert!((octave - 496.0).abs() < 1.0);
    }

    #[test]
    fn circuit_vca_is_monotonic_and_has_a_short_analog_slew() {
        let mut previous = 0.0;
        for step in 0..=100 {
            let gain = nominal_vca_gain(step as f32 / 100.0);
            assert!(gain >= previous);
            previous = gain;
        }
        assert_eq!(nominal_vca_gain(0.0), 0.0);
        assert!((nominal_vca_gain(1.0) - 1.0).abs() < 1.0e-6);
        let mut chip = A1qH80017a::new();
        let first = chip.process_vca(1.0, 48_000.0, 1.0);
        assert!(first > 0.0 && first < 0.1);
        for _ in 0..480 {
            chip.process_vca(1.0, 48_000.0, 1.0);
        }
        assert!(chip.vca_gain() > 0.99);
    }

    #[test]
    fn analog_coefficient_caches_are_sample_exact() {
        let mut cached = A1qH80017a::with_seed(0x1234_5678);
        for _ in 0..64 {
            let filtered = cached.process_vcf(0.25, 48_000.0, 8_192, 6_144);
            let _ = cached.process_vca(filtered, 48_000.0, 0.625);
        }

        let mut rebuilt = cached;
        rebuilt.vcf_sample_rate = 0.0;
        rebuilt.vca_sample_rate = 0.0;

        let cached_filtered = cached.process_vcf(-0.125, 48_000.0, 8_192, 6_144);
        let rebuilt_filtered = rebuilt.process_vcf(-0.125, 48_000.0, 8_192, 6_144);
        assert_eq!(cached_filtered.to_bits(), rebuilt_filtered.to_bits());

        let cached_output = cached.process_vca(cached_filtered, 48_000.0, 0.625);
        let rebuilt_output = rebuilt.process_vca(rebuilt_filtered, 48_000.0, 0.625);
        assert_eq!(cached_output.to_bits(), rebuilt_output.to_bits());
    }

    #[test]
    fn analog_voice_cell_stays_finite_across_extreme_controls() {
        let mut chip = A1qH80017a::new();
        for index in 0..96_000 {
            let control = if index & 1 == 0 { 0 } else { DAC_MAX };
            let input = if index % 3 == 0 { 4.0 } else { -4.0 };
            let resonance = if index & 2 == 0 { 0 } else { DAC_MAX };
            let output = chip.process_vcf(input, 48_000.0, control, resonance);
            assert!(output.is_finite());
        }
        assert!(chip.filter_state().iter().all(|value| value.is_finite()));
    }

    #[test]
    fn thermal_excitation_starts_calibrated_self_oscillation_but_stays_subaudible_at_zero_q() {
        let mut quiet = A1qH80017a::new();
        let mut quiet_peak = 0.0_f32;
        for _ in 0..48_000 {
            quiet_peak = quiet_peak.max(
                quiet
                    .process_vcf(0.0, 48_000.0, dac_word(VCF_CALIBRATION_WORD), 0)
                    .abs(),
            );
        }
        assert!(quiet_peak < 1.0e-5, "quiet_peak={quiet_peak}");

        let mut resonant = A1qH80017a::new();
        let mut peak = 0.0_f32;
        let mut previous = 0.0_f32;
        let mut first_crossing = None;
        let mut last_crossing = 0_usize;
        let mut crossings = 0_usize;
        for sample in 0..144_000 {
            let output = resonant.process_vcf(
                0.0,
                48_000.0,
                dac_word(VCF_CALIBRATION_WORD),
                PANEL_LEVEL_DAC_MAX,
            );
            if sample >= 48_000 {
                peak = peak.max(output.abs());
                if previous <= 0.0 && output > 0.0 {
                    first_crossing.get_or_insert(sample);
                    last_crossing = sample;
                    crossings += 1;
                }
            }
            previous = output;
        }
        let first_crossing = first_crossing.unwrap();
        let frequency = (crossings - 1) as f32 * 48_000.0 / (last_crossing - first_crossing) as f32;
        assert!(peak > 0.015, "peak={peak}");
        assert!(
            (frequency - VCF_CALIBRATION_HZ).abs() < 2.0,
            "frequency={frequency}"
        );
    }

    #[test]
    fn analog_cell_noise_is_reproducible_per_seed_and_reset() {
        let mut chip = A1qH80017a::with_seed(0x1234_5678);
        let first = core::array::from_fn::<_, 16, _>(|_| {
            chip.process_vcf(
                0.0,
                48_000.0,
                dac_word(VCF_CALIBRATION_WORD),
                PANEL_LEVEL_DAC_MAX,
            )
        });
        for _ in 0..64 {
            chip.process_vcf(
                0.0,
                48_000.0,
                dac_word(VCF_CALIBRATION_WORD),
                PANEL_LEVEL_DAC_MAX,
            );
        }
        chip.reset();
        let repeated = core::array::from_fn::<_, 16, _>(|_| {
            chip.process_vcf(
                0.0,
                48_000.0,
                dac_word(VCF_CALIBRATION_WORD),
                PANEL_LEVEL_DAC_MAX,
            )
        });
        assert_eq!(first, repeated);

        let mut other = A1qH80017a::with_seed(0x8765_4321);
        let other_first = other.process_vcf(
            0.0,
            48_000.0,
            dac_word(VCF_CALIBRATION_WORD),
            PANEL_LEVEL_DAC_MAX,
        );
        assert_ne!(first[0], other_first);
    }

    #[test]
    fn vcf_substep_satisfies_the_zero_delay_four_ota_loop() {
        let retained = [0.12, -0.08, 0.04, -0.02];
        let mut states = retained;
        let input = 0.73;
        let coefficient = 0.31;
        let feedback = 3.7;
        let output = process_vcf_substep(&mut states, input, coefficient, feedback);
        let stage_outputs =
            core::array::from_fn::<_, 4, _>(|index| (states[index] + retained[index]) * 0.5);
        assert!((output - stage_outputs[3]).abs() < 1.0e-7);

        let mut stage_input = input - feedback * stage_outputs[3];
        for index in 0..4 {
            let expected =
                retained[index] + coefficient * (ota_saturate(stage_input) - retained[index]);
            assert!(
                (stage_outputs[index] - expected).abs() < 1.0e-6,
                "stage={index} actual={} expected={expected}",
                stage_outputs[index]
            );
            stage_input = stage_outputs[index];
        }
    }

    #[test]
    fn ota_slope_matches_the_differential_pair_transfer() {
        let epsilon = 1.0e-3;
        for value in [-2.5_f32, -1.0, 0.0, 0.75, 2.5] {
            let (_, slope) = ota_saturate_with_slope(value);
            let numerical =
                (ota_saturate(value + epsilon) - ota_saturate(value - epsilon)) / (2.0 * epsilon);
            assert!(
                (slope - numerical).abs() < 2.0e-4,
                "value={value} slope={slope} numerical={numerical}"
            );
        }
    }
}
