#![no_std]

use rf_106_chorus::{ChorusMode, StereoChorus};
use rf_106_contract::{
    NATIVE_DCO_LFO as PARAM_DCO_LFO, NATIVE_DCO_NOISE_LEVEL as PARAM_DCO_NOISE_LEVEL,
    NATIVE_DCO_PULSE as PARAM_DCO_PULSE, NATIVE_DCO_PWM as PARAM_DCO_PWM_DEPTH,
    NATIVE_DCO_RANGE as PARAM_DCO_RANGE, NATIVE_DCO_SAW as PARAM_DCO_SAW,
    NATIVE_DCO_SUB_LEVEL as PARAM_DCO_SUB_LEVEL, NATIVE_HPF as PARAM_HPF,
    NATIVE_LFO_DELAY as PARAM_LFO_DELAY, NATIVE_LFO_RATE as PARAM_LFO_RATE, NATIVE_PARAMETER_COUNT,
    NATIVE_PWM_MODE as PARAM_PWM_MODE, NATIVE_VCA_LEVEL as PARAM_OUTPUT_VCA,
    NATIVE_VCA_MODE as PARAM_VCA_MODE, NATIVE_VCF_ENV_POLARITY as PARAM_VCF_ENV_POLARITY,
    factory_preset, native_parameter_value_is_valid, sysex::Tone,
};
use rf_106_control::{
    AllocationMode, GlobalLfo, KEYBOARD_KEY_COUNT, KeyTranspose, NoteSource, PerformanceState,
    Portamento, VOICE_COUNT, VoiceAction, VoiceAllocator, combine_dco_lfo_depth,
    dco_lfo_destination_from_coefficient, vcf_lfo_destination,
};
use rf_106_output::{HpfPosition, M5218Summer, OutputPath, bender_board_volume};
use rf_106_voice::{
    A1qH80017a, CONTROL_CYCLE_SECONDS, Dco, DcoRange, Envelope, EnvelopeStage, NoiseSource,
    PwmMode, SourceControl, VcaMode, VcfControl, VcfEnvelopePolarity, resonance_dac, vca_control,
};

const PARAM_BENDER_DCO: usize = 0;
const PARAM_BENDER_VCF: usize = 1;
const PARAM_TUNING: usize = 37;
const PARAM_POWER: usize = 38;
const PARAM_ALLOCATION_MODE: usize = 39;
const PARAM_PORTAMENTO: usize = 40;
const PARAM_KEY_TRANSPOSE: usize = 41;
const PARAM_BENDER_LFO: usize = 42;
const PARAM_HOST_OUTPUT: usize = 44;
/// A physical program change and the next key scan cannot occur at the same
/// zero-time instant: the six DCO/VCF cells keep running behind closed VCAs.
/// Three measured B_2 loops are the current evidence-bounded settling window;
/// raw hardware timing remains an E4 calibration item.
const PROGRAM_CHANGE_SETTLING_CYCLES: f32 = 3.0;

#[derive(Clone, Copy)]
struct Voice {
    note: u8,
    active: bool,
    gate: bool,
    sustained: bool,
    dco: Dco,
    envelope: Envelope,
    vcf_control: VcfControl,
    velocity: f32,
    analog_cell: A1qH80017a,
    portamento: Portamento,
}

impl Voice {
    const fn silent(slot: usize) -> Self {
        Self {
            // B_2 cold-starts FF09..FF0E and FF71..FF7C at middle C.
            note: 60,
            active: false,
            gate: false,
            sustained: false,
            dco: Dco::new(),
            envelope: Envelope::new(),
            vcf_control: VcfControl::new(),
            velocity: 0.0,
            // Physical voice cells have independent microscopic noise. Fixed
            // per-slot seeds preserve that distinction reproducibly.
            analog_cell: A1qH80017a::with_seed(0x075b_cd15 ^ ((slot as u32 + 1) * 0x0019_660d)),
            portamento: Portamento::new(),
        }
    }
}

#[derive(Clone, Copy)]
struct VoiceControlFrame {
    dco_bend_semitones: f32,
    vcf_bend_word: i16,
    dco_lfo_8_8: i16,
    vcf_lfo_word: i16,
    pwm_lfo_source: u16,
    noise: f32,
}

pub struct Synth {
    parameters: [f64; NATIVE_PARAMETER_COUNT],
    voices: [Voice; VOICE_COUNT],
    allocator: VoiceAllocator,
    sample_rate: f32,
    prepared: bool,
    sustain: bool,
    performance: PerformanceState,
    lfo: GlobalLfo,
    noise: NoiseSource,
    summer: M5218Summer,
    output: OutputPath,
    chorus: StereoChorus,
    physical_volume: f64,
    portamento_switch_on: bool,
    key_transpose: KeyTranspose,
    keyboard_notes: [Option<u8>; KEYBOARD_KEY_COUNT as usize],
}

impl Default for Synth {
    fn default() -> Self {
        let mut synth = Self {
            parameters: [0.0; NATIVE_PARAMETER_COUNT],
            voices: core::array::from_fn(Voice::silent),
            allocator: VoiceAllocator::new(),
            sample_rate: 48_000.0,
            prepared: false,
            sustain: false,
            performance: PerformanceState::new(),
            lfo: GlobalLfo::new(),
            noise: NoiseSource::new(),
            summer: M5218Summer::new(),
            output: OutputPath::new(),
            chorus: StereoChorus::new(),
            // Physical performance control, independent of patch memory.
            physical_volume: 1.0,
            // Preserves the pre-switch engine behavior for existing sessions.
            // The physical time knob still defaults to zero (instantaneous).
            portamento_switch_on: true,
            key_transpose: KeyTranspose::new(),
            keyboard_notes: [None; KEYBOARD_KEY_COUNT as usize],
        };
        synth.parameters[PARAM_BENDER_DCO] = 1.0;
        synth.parameters[PARAM_BENDER_LFO] = 0.5;
        synth.parameters[PARAM_ALLOCATION_MODE] = AllocationMode::Poly1.native_parameter();
        // RackForge-only trim: new instances start at unity. The physical
        // RF-106 volume law is modeled separately after the chorus.
        synth.parameters[PARAM_HOST_OUTPUT] = 1.0;
        let _ = synth.load_factory_preset(0);
        synth
    }
}

impl Synth {
    pub fn prepare(&mut self, sample_rate: f64) -> bool {
        if !sample_rate.is_finite() || !(8_000.0..=192_000.0).contains(&sample_rate) {
            return false;
        }
        self.sample_rate = sample_rate as f32;
        self.prepared = true;
        self.performance.prepare(self.sample_rate);
        self.lfo.prepare(self.sample_rate);
        self.noise.prepare(self.sample_rate);
        self.reset();
        self.settle_idle_program_change();
        true
    }

    pub fn reset(&mut self) {
        self.voices = core::array::from_fn(Voice::silent);
        self.allocator.reset();
        let actions = self
            .allocator
            .set_mode(AllocationMode::from_native_parameter(
                self.parameters[PARAM_ALLOCATION_MODE],
            ));
        self.apply_voice_actions(actions);
        self.sustain = false;
        self.performance.reset();
        self.lfo.reset();
        self.noise.reset();
        self.summer.reset();
        self.output.reset();
        self.chorus.reset();
        self.key_transpose = KeyTranspose::from_offset(self.parameters[PARAM_KEY_TRANSPOSE] as i8)
            .unwrap_or_default();
        self.keyboard_notes.fill(None);
    }

    pub fn parameters(&self) -> &[f64; NATIVE_PARAMETER_COUNT] {
        &self.parameters
    }

    pub fn parameter(&self, index: u32) -> Option<f64> {
        self.parameters.get(index as usize).copied()
    }

    pub fn set_parameter(&mut self, index: u32, value: f64) -> bool {
        if !native_parameter_value_is_valid(index, value) {
            return false;
        }
        self.parameters[index as usize] = value;
        if index as usize == PARAM_KEY_TRANSPOSE {
            self.key_transpose = KeyTranspose::from_offset(value as i8).unwrap_or_default();
        }
        if index as usize == PARAM_ALLOCATION_MODE {
            let actions = self
                .allocator
                .set_mode(AllocationMode::from_native_parameter(value));
            self.apply_voice_actions(actions);
        }
        true
    }

    pub fn load_parameters(&mut self, values: &[f64; NATIVE_PARAMETER_COUNT]) -> bool {
        if values
            .iter()
            .enumerate()
            .any(|(index, value)| !native_parameter_value_is_valid(index as u32, *value))
        {
            return false;
        }
        self.parameters = *values;
        self.reset();
        self.settle_idle_program_change();
        true
    }

    pub fn load_factory_preset(&mut self, index: u32) -> bool {
        let Some(preset) = factory_preset(index) else {
            return false;
        };
        if preset
            .values
            .iter()
            .enumerate()
            .any(|(index, value)| !native_parameter_value_is_valid(index as u32, *value))
        {
            return false;
        }
        for (index, value) in preset.values.iter().copied().enumerate() {
            if !matches!(
                index,
                PARAM_BENDER_DCO
                    | PARAM_BENDER_VCF
                    | PARAM_TUNING
                    | PARAM_ALLOCATION_MODE
                    | PARAM_PORTAMENTO
                    | PARAM_KEY_TRANSPOSE
                    | PARAM_BENDER_LFO
                    | PARAM_HOST_OUTPUT
            ) {
                self.parameters[index] = value;
            }
        }
        // The imported table retains the original SysEx byte divided by 127.
        // Re-applying its canonical tone fixes the JUNO-106 PWM range (0..105)
        // and keeps the historical PWM/SUB order explicit.
        Tone::from_factory(preset).apply_to_native(&mut self.parameters);
        self.settle_idle_program_change();
        true
    }

    /// Load the eighteen controls carried by an original JUNO-106 tone dump.
    /// Physical performance controls and RackForge-only settings survive it.
    pub fn load_tone(&mut self, tone: &Tone) -> bool {
        let mut parameters = self.parameters;
        tone.apply_to_native(&mut parameters);
        if parameters
            .iter()
            .enumerate()
            .any(|(index, value)| !native_parameter_value_is_valid(index as u32, *value))
        {
            return false;
        }
        self.parameters = parameters;
        self.settle_idle_program_change();
        true
    }

    pub fn note_on(&mut self, note: u8, velocity: u8) {
        self.note_on_from(NoteSource::Midi, note, velocity);
    }

    /// Selects Key Transpose through the same button-held keyboard gesture as
    /// A_5. This performance state is independent of patch selection.
    pub fn select_key_transpose_key(&mut self, key_index: u8) -> bool {
        if !self.key_transpose.select_key(key_index) {
            return false;
        }
        self.parameters[PARAM_KEY_TRANSPOSE] = f64::from(self.key_transpose.offset());
        true
    }

    pub const fn key_transpose_offset(&self) -> i8 {
        self.key_transpose.offset()
    }

    pub const fn key_transpose_indicator(&self) -> bool {
        self.key_transpose.indicator()
    }

    /// Physical key-matrix input. RackForge MIDI events intentionally do not
    /// call this path because the RF-106 manual and A_5 both bypass Key
    /// Transpose for MIDI IN.
    pub fn keyboard_key_on(&mut self, key_index: u8, velocity: u8) -> bool {
        if velocity == 0 {
            return self.keyboard_key_off(key_index);
        }
        let Some(slot) = self.keyboard_notes.get_mut(key_index as usize) else {
            return false;
        };
        if slot.is_some() {
            return true;
        }
        let Some(note) = self.key_transpose.voice_note(key_index) else {
            return false;
        };
        *slot = Some(note);
        self.note_on_from(NoteSource::Keyboard, note, velocity);
        true
    }

    pub fn keyboard_key_off(&mut self, key_index: u8) -> bool {
        let Some(slot) = self.keyboard_notes.get_mut(key_index as usize) else {
            return false;
        };
        let Some(note) = slot.take() else {
            return true;
        };
        self.note_off_from(NoteSource::Keyboard, note);
        true
    }

    pub fn note_on_from(&mut self, source: NoteSource, note: u8, velocity: u8) {
        if velocity == 0 {
            self.note_off_from(source, note);
            return;
        }
        let actions = self.allocator.note_on(source, note, velocity);
        self.apply_voice_actions(actions);
    }

    pub fn note_off(&mut self, note: u8) {
        self.note_off_from(NoteSource::Midi, note);
    }

    pub fn note_off_from(&mut self, source: NoteSource, note: u8) {
        let actions = self.allocator.note_off(source, note);
        self.apply_voice_actions(actions);
    }

    pub fn control_change(&mut self, controller: u8, value: u8) {
        match controller {
            1 => self.performance.set_modulation(value),
            64 => self.set_hold(value != 0),
            _ => {}
        }
    }

    pub fn set_hold(&mut self, enabled: bool) {
        if self.sustain && !enabled {
            for voice in &mut self.voices {
                if voice.sustained {
                    voice.sustained = false;
                    voice.envelope.note_off();
                }
            }
        }
        self.sustain = enabled;
    }

    pub fn set_pitch_bend(&mut self, value: u16) {
        self.performance.set_pitch_bend(value);
    }

    pub fn pitch_bend_position(&self) -> f32 {
        self.performance.bend.unit()
    }

    /// Drives the physical Bender Board LFO Trigger gesture. MIDI CC1 remains
    /// an independent, immediate path as it is in the Voice firmware.
    pub fn set_lfo_trigger(&mut self, active: bool) {
        self.performance.set_lfo_trigger(active);
    }

    pub const fn lfo_trigger_active(&self) -> bool {
        self.performance.lfo_trigger_active
    }

    /// Bender Board SW1. This is a performance control, not patch memory: it
    /// connects VR2's wiper to Voice ADC AN1 when on and lets R16 ground AN1
    /// when off.
    pub fn set_portamento_switch(&mut self, switch_on: bool) {
        self.portamento_switch_on = switch_on;
    }

    pub const fn portamento_switch_on(&self) -> bool {
        self.portamento_switch_on
    }

    /// Physical dual-gang Bender Board VR1, after the stereo chorus.
    pub fn set_physical_volume(&mut self, value: f64) -> bool {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return false;
        }
        self.physical_volume = value;
        true
    }

    pub const fn physical_volume(&self) -> f64 {
        self.physical_volume
    }

    pub fn all_notes_off(&mut self) {
        let actions = self.allocator.all_notes_off();
        self.apply_voice_actions(actions);
    }

    /// Performs the musical reset associated with the MIDI Channel
    /// selection gesture completes: Hold is released, notes are turned off,
    /// and MIDI bend/modulation return to their neutral values. The physical
    /// Bender Board trigger remains an independent analog input.
    pub fn reset_for_midi_channel_selection(&mut self) {
        self.set_hold(false);
        self.all_notes_off();
        self.performance.set_pitch_bend(0x2000);
        self.performance.set_modulation(0);
    }

    pub fn allocator_gate_mask(&self) -> u8 {
        self.allocator.gate_mask()
    }

    /// Renders the emulated instrument through the physical Bender Board
    /// volume wipers after the stereo chorus.
    ///
    /// No RackForge gain, sanitization or limiting is applied here. Keeping
    /// those host policies outside this method makes circuit captures and
    /// regression renders comparable without an undocumented post-process.
    pub fn process_circuit_sample(&mut self) -> (f32, f32) {
        if self.parameters[PARAM_POWER] < 0.5 {
            return (0.0, 0.0);
        }
        self.lfo.process(
            self.parameters[PARAM_LFO_RATE] as f32,
            self.parameters[PARAM_LFO_DELAY] as f32,
            self.voices
                .iter()
                .any(|voice| voice.gate || voice.sustained),
        );
        // The Module Board owns one selected-transistor/BA662 source. Its one
        // sample is distributed to every voice, just like the physical bus.
        let noise = self.noise.process();
        let parameters = self.parameters;
        let portamento_switch_on = self.portamento_switch_on;
        let sample_rate = self.sample_rate;
        let dco_bend = self
            .performance
            .bend
            .dco_semitones(parameters[PARAM_BENDER_DCO] as f32);
        let vcf_bend_magnitude =
            self.performance
                .bend
                .vcf_offset_word(parameters[PARAM_BENDER_VCF] as f32) as i16;
        let vcf_bend_word = if self.performance.bend.negative {
            -vcf_bend_magnitude
        } else {
            vcf_bend_magnitude
        };
        let performance_lfo = self
            .performance
            .process_lfo_depth(parameters[PARAM_BENDER_LFO] as f32);
        let combined_dco_depth = combine_dco_lfo_depth(
            panel_control_raw(parameters[PARAM_DCO_LFO] as f32),
            self.lfo.amplitude(),
            performance_lfo,
        );
        let dco_lfo_8_8 = signed_lfo_destination(
            dco_lfo_destination_from_coefficient(self.lfo.accumulator(), combined_dco_depth),
            self.lfo.is_negative(),
        );
        let vcf_lfo_word = signed_lfo_destination(
            vcf_lfo_destination(
                self.lfo.accumulator(),
                self.lfo.amplitude(),
                panel_control_raw(parameters[13] as f32),
            ),
            self.lfo.is_negative(),
        );
        let control = VoiceControlFrame {
            dco_bend_semitones: dco_bend,
            vcf_bend_word,
            dco_lfo_8_8,
            vcf_lfo_word,
            pwm_lfo_source: self.lfo.pwm_source(),
            noise,
        };
        let mut voice_outputs = [0.0; VOICE_COUNT];
        for (output, voice) in voice_outputs.iter_mut().zip(&mut self.voices) {
            *output = render_voice(
                voice,
                &parameters,
                sample_rate,
                portamento_switch_on,
                control,
            );
        }
        let mut mono = self.summer.process(&voice_outputs, sample_rate);
        mono = self.output.process(
            mono,
            sample_rate,
            HpfPosition::from_native(parameters[PARAM_HPF]),
            parameters[PARAM_OUTPUT_VCA] as f32,
        );
        self.chorus.set_mode(ChorusMode::from_switches(
            parameters[27] >= 0.5,
            parameters[28] >= 0.5,
        ));
        let stereo = self.chorus.process(mono, self.sample_rate);
        bender_board_volume(stereo, self.physical_volume as f32)
    }

    /// Advances only an idle instrument through the real program-selection
    /// interval. This is not a preset-specific fade or gain: it preserves the
    /// continuously biased DCO, IR3109 and BA662 state that exists before a
    /// key is scanned. A held/releasing performance is never fast-forwarded.
    fn settle_idle_program_change(&mut self) {
        if !self.prepared
            || self
                .voices
                .iter()
                .any(|voice| voice.active || voice.gate || voice.sustained)
        {
            return;
        }
        let samples =
            libm::ceilf(self.sample_rate * CONTROL_CYCLE_SECONDS * PROGRAM_CHANGE_SETTLING_CYCLES)
                as u32;
        for _ in 0..samples {
            let _ = self.process_circuit_sample();
        }
    }

    fn apply_voice_actions(&mut self, actions: rf_106_control::ActionBatch) {
        for action in actions.iter() {
            match action {
                VoiceAction::GateOn {
                    voice,
                    note,
                    velocity,
                } => self.gate_voice_on(voice, note, velocity),
                VoiceAction::GateOff { voice } => self.gate_voice_off(voice),
            }
        }
    }

    fn gate_voice_on(&mut self, slot: usize, note: u8, _velocity: u8) {
        let dco = self.voices[slot].dco;
        let mut envelope = self.voices[slot].envelope;
        let vcf_control = self.voices[slot].vcf_control;
        let analog_cell = self.voices[slot].analog_cell;
        if !self.voices[slot].active {
            envelope.reset();
        }
        envelope.note_on();
        let portamento = self.voices[slot].portamento;
        self.voices[slot] = Voice {
            note,
            active: true,
            gate: true,
            sustained: false,
            dco,
            envelope,
            vcf_control,
            // The RF-106 keyboard and VCA are not velocity-sensitive. MIDI
            // velocity only distinguishes Note On from velocity-zero Note Off.
            velocity: 1.0,
            // The physical 80017A is continuously biased; a new key does not
            // reset its four capacitors or thermal-noise phase.
            analog_cell,
            portamento,
        };
    }

    fn gate_voice_off(&mut self, slot: usize) {
        let voice = &mut self.voices[slot];
        if !voice.active || !voice.gate {
            return;
        }
        voice.gate = false;
        if self.sustain {
            voice.sustained = true;
        } else {
            voice.envelope.note_off();
        }
    }
}

fn render_voice(
    voice: &mut Voice,
    parameters: &[f64; NATIVE_PARAMETER_COUNT],
    sample_rate: f32,
    portamento_switch_on: bool,
    control: VoiceControlFrame,
) -> f32 {
    // B_2 advances all six persistent 8.8 pitch accumulators every foreground
    // cycle, including voices whose VCA is currently closed.
    let base_pitch_8_8 = voice.portamento.process(
        sample_rate,
        voice.note,
        parameters[PARAM_PORTAMENTO] as f32,
        portamento_switch_on,
    );
    let envelope = if voice.active {
        let envelope = voice.envelope.process(
            sample_rate,
            parameters[16] as f32,
            parameters[17] as f32,
            parameters[18] as f32,
            parameters[19] as f32,
        );
        if voice.envelope.stage() == EnvelopeStage::Idle {
            voice.active = false;
        }
        envelope
    } else {
        0.0
    };

    let tuning = parameters[PARAM_TUNING] as f32;
    let base_pitch = f32::from(base_pitch_8_8) / 256.0;
    let lfo_pitch = f32::from(control.dco_lfo_8_8) / 256.0;
    let midi = base_pitch + tuning + control.dco_bend_semitones + lfo_pitch;
    // The real sub-oscillator source is controlled solely by its level DAC.
    let sources = SourceControl::from_native(
        parameters[PARAM_DCO_SAW] >= 0.5,
        parameters[PARAM_DCO_PULSE] >= 0.5,
        parameters[PARAM_DCO_PWM_DEPTH] as f32,
        parameters[PARAM_DCO_NOISE_LEVEL] as f32,
        parameters[PARAM_DCO_SUB_LEVEL] as f32,
        if parameters[PARAM_PWM_MODE] < 0.5 {
            PwmMode::Lfo
        } else {
            PwmMode::Manual
        },
    );
    let pulse_width = sources.pulse_duty(control.pwm_lfo_source);
    let note_8_8 = libm::roundf(midi.clamp(0.0, 255.996) * 256.0) as u16;
    let range = match parameters[PARAM_DCO_RANGE] as u8 {
        0 => DcoRange::SixteenFoot,
        2 => DcoRange::FourFoot,
        _ => DcoRange::EightFoot,
    };
    let dco = voice.dco.process(sample_rate, note_8_8, range, pulse_width);
    let oscillator = sources.mix(dco, control.noise);

    let lfo_offset = control.vcf_lfo_word;
    let bend_offset = control.vcf_bend_word;
    let cutoff_word = voice.vcf_control.process(
        sample_rate,
        parameters[10] as f32,
        voice.envelope.value(),
        parameters[12] as f32,
        vcf_envelope_polarity(parameters[PARAM_VCF_ENV_POLARITY]),
        parameters[14] as f32,
        note_8_8,
        lfo_offset,
        bend_offset,
    );
    let filtered = voice.analog_cell.process_vcf(
        oscillator,
        sample_rate,
        cutoff_word.dac,
        resonance_dac(parameters[11] as f32),
    );

    let velocity = if !voice.active || parameters[47] >= 0.5 {
        1.0
    } else {
        0.25 + voice.velocity * 0.75
    };
    let vca = vca_control(
        if parameters[PARAM_VCA_MODE] >= 0.5 {
            VcaMode::Gate
        } else {
            VcaMode::Envelope
        },
        envelope,
        voice.gate || voice.sustained,
    );
    voice
        .analog_cell
        .process_vca(filtered, sample_rate, vca * velocity)
}

const fn signed_lfo_destination(magnitude: u16, negative: bool) -> i16 {
    if negative {
        -(magnitude as i16)
    } else {
        magnitude as i16
    }
}

fn panel_control_raw(value: f32) -> u8 {
    libm::roundf(value.clamp(0.0, 1.0) * 127.0) as u8
}

/// The public slot is named `VCF Env Inv`, while B_2 switches-2 bit 1 means
/// the opposite physical state: positive envelope modulation.
const fn vcf_envelope_polarity(invert_parameter: f64) -> VcfEnvelopePolarity {
    if invert_parameter >= 0.5 {
        VcfEnvelopePolarity::Negative
    } else {
        VcfEnvelopePolarity::Positive
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_factory_preset_is_accepted() {
        let mut synth = Synth::default();
        for index in 0..rf_106_contract::FACTORY_PRESET_COUNT as u32 {
            assert!(synth.load_factory_preset(index), "preset {index}");
        }
    }

    #[test]
    fn tomita_factory_patch_keeps_its_original_pwm_and_self_oscillating_filter() {
        const TOMITA_B35: u32 = 84;
        let preset = factory_preset(TOMITA_B35).expect("B35 must exist");
        assert_eq!(preset.name, "B35 Tomita");
        assert!(
            preset.values[PARAM_DCO_PWM_DEPTH] > 0.8,
            "PWM must be raised"
        );
        assert_eq!(
            preset.values[PARAM_DCO_SUB_LEVEL], 0.0,
            "the original SUB level is zero"
        );
        assert!(preset.values[11] > 0.98, "the resonant VCF is the source");
        assert_eq!(
            preset.values[PARAM_DCO_NOISE_LEVEL], 0.0,
            "noise is not an audible source"
        );
        assert_eq!(
            preset.values[PARAM_DCO_PULSE], 0.0,
            "pulse is not an audible source"
        );
        assert_eq!(
            preset.values[PARAM_DCO_SAW], 0.0,
            "saw is not an audible source"
        );
        let mut synth = Synth::default();
        assert!(synth.prepare(48_000.0));
        assert!(synth.load_factory_preset(TOMITA_B35));
        synth.note_on(60, 127);

        let mut peak = 0.0_f32;
        let mut sum_squares = 0.0_f64;
        for _ in 0..48_000 {
            let (left, right) = synth.process_circuit_sample();
            assert!(left.is_finite() && right.is_finite());
            peak = peak.max(left.abs()).max(right.abs());
            sum_squares += f64::from(left * left + right * right);
        }
        let rms = (sum_squares / 96_000.0).sqrt();
        assert!(peak > 0.04, "B35 peak={peak}");
        assert!(rms > 0.01, "B35 rms={rms}");
    }

    #[test]
    fn public_env_invert_maps_to_the_opposite_physical_switch_meaning() {
        assert_eq!(vcf_envelope_polarity(0.0), VcfEnvelopePolarity::Positive);
        assert_eq!(vcf_envelope_polarity(1.0), VcfEnvelopePolarity::Negative);
    }

    #[test]
    fn note_produces_finite_audio_and_releases() {
        let mut synth = Synth::default();
        assert!(synth.prepare(48_000.0));
        synth.note_on(60, 100);
        let mut peak = 0.0_f32;
        for _ in 0..4096 {
            let (left, right) = synth.process_circuit_sample();
            assert!(left.is_finite() && right.is_finite());
            peak = peak.max(left.abs()).max(right.abs());
        }
        assert!(peak > 0.0001, "peak={peak}");
        synth.note_off(60);
        for _ in 0..500_000 {
            let _ = synth.process_circuit_sample();
        }
        assert!(synth.voices.iter().all(|voice| !voice.active));
    }

    #[test]
    fn circuit_render_is_independent_of_the_rackforge_master_slot() {
        let mut quiet_host = Synth::default();
        let mut loud_host = Synth::default();
        assert!(quiet_host.prepare(48_000.0));
        assert!(loud_host.prepare(48_000.0));
        assert!(quiet_host.set_parameter(44, 0.0));
        assert!(loud_host.set_parameter(44, 1.0));
        quiet_host.note_on(60, 100);
        loud_host.note_on(60, 100);

        for _ in 0..4_096 {
            assert_eq!(
                quiet_host.process_circuit_sample(),
                loud_host.process_circuit_sample()
            );
        }
    }

    #[test]
    fn rackforge_host_output_starts_at_unity() {
        assert_eq!(
            Synth::default().parameter(PARAM_HOST_OUTPUT as u32),
            Some(1.0)
        );
    }

    #[test]
    fn physical_volume_is_the_final_circuit_gain_and_reaches_true_zero() {
        let mut closed = Synth::default();
        let mut half = Synth::default();
        let mut open = Synth::default();
        for synth in [&mut closed, &mut half, &mut open] {
            assert!(synth.prepare(48_000.0));
            synth.note_on(60, 100);
        }
        assert!(closed.set_physical_volume(0.0));
        assert!(half.set_physical_volume(0.5));
        assert!(open.set_physical_volume(1.0));
        assert!(!open.set_physical_volume(f64::NAN));
        assert!(!open.set_physical_volume(1.01));

        let gain = rf_106_output::bender_board_volume_gain(0.5);
        for _ in 0..4_096 {
            assert_eq!(closed.process_circuit_sample(), (0.0, 0.0));
            let middle = half.process_circuit_sample();
            let full = open.process_circuit_sample();
            assert!((middle.0 - full.0 * gain).abs() < 1.0e-7);
            assert!((middle.1 - full.1 * gain).abs() < 1.0e-7);
        }
    }

    #[test]
    fn factory_patch_changes_preserve_hardware_performance_controls() {
        let mut synth = Synth::default();
        assert!(synth.set_parameter(PARAM_BENDER_DCO as u32, 0.75));
        assert!(synth.set_parameter(PARAM_BENDER_VCF as u32, 0.25));
        assert!(synth.set_parameter(PARAM_BENDER_LFO as u32, 0.6));
        assert!(synth.set_parameter(PARAM_TUNING as u32, -0.35));
        assert!(synth.set_parameter(PARAM_PORTAMENTO as u32, 0.8));
        assert!(synth.set_parameter(PARAM_KEY_TRANSPOSE as u32, 7.0));
        synth.set_portamento_switch(false);
        assert!(synth.set_physical_volume(0.42));
        assert!(synth.set_parameter(PARAM_HOST_OUTPUT as u32, 0.73));
        assert!(synth.load_factory_preset(52));
        assert_eq!(synth.parameter(PARAM_BENDER_DCO as u32), Some(0.75));
        assert_eq!(synth.parameter(PARAM_BENDER_VCF as u32), Some(0.25));
        assert_eq!(synth.parameter(PARAM_BENDER_LFO as u32), Some(0.6));
        assert_eq!(synth.parameter(PARAM_TUNING as u32), Some(-0.35));
        assert_eq!(synth.parameter(PARAM_PORTAMENTO as u32), Some(0.8));
        assert_eq!(synth.parameter(PARAM_KEY_TRANSPOSE as u32), Some(7.0));
        assert_eq!(synth.key_transpose_offset(), 7);
        assert!(!synth.portamento_switch_on());
        assert_eq!(synth.physical_volume(), 0.42);
        assert_eq!(synth.parameter(PARAM_HOST_OUTPUT as u32), Some(0.73));
    }

    #[test]
    fn portamento_switch_is_independent_from_the_time_knob() {
        let mut synth = Synth::default();
        assert!(synth.portamento_switch_on());
        assert!(synth.set_parameter(PARAM_PORTAMENTO as u32, 0.875));
        synth.set_portamento_switch(false);
        assert!(!synth.portamento_switch_on());
        assert_eq!(synth.parameter(PARAM_PORTAMENTO as u32), Some(0.875));
        synth.set_portamento_switch(true);
        assert!(synth.portamento_switch_on());
        assert_eq!(synth.parameter(PARAM_PORTAMENTO as u32), Some(0.875));
    }

    #[test]
    fn key_transpose_affects_keyboard_but_bypasses_midi_in() {
        let mut synth = Synth::default();
        assert!(!synth.key_transpose_indicator());
        assert!(synth.select_key_transpose_key(31));
        assert_eq!(synth.key_transpose_offset(), 7);
        assert!(synth.key_transpose_indicator());

        assert!(synth.keyboard_key_on(0, 100));
        assert_eq!(synth.voices[0].note, 43);
        assert_eq!(synth.allocator_gate_mask(), 1);

        synth.note_on(36, 100);
        assert_eq!(synth.voices[1].note, 36);
        assert_eq!(synth.allocator_gate_mask(), 3);
        synth.note_off(36);
        assert_eq!(synth.allocator_gate_mask(), 1);
        assert!(synth.keyboard_key_off(0));
        assert_eq!(synth.allocator_gate_mask(), 0);

        assert!(synth.select_key_transpose_key(KeyTranspose::MIDDLE_C_KEY));
        assert_eq!(synth.key_transpose_offset(), 0);
        assert!(!synth.key_transpose_indicator());
        assert!(synth.keyboard_key_on(0, 100));
        assert!(
            synth
                .voices
                .iter()
                .any(|voice| voice.gate && voice.note == 36)
        );
    }

    #[test]
    fn keyboard_release_uses_the_offset_captured_on_key_down() {
        let mut synth = Synth::default();
        assert!(synth.select_key_transpose_key(31));
        assert!(synth.keyboard_key_on(0, 100));
        assert_eq!(synth.allocator_gate_mask(), 1);
        assert!(synth.select_key_transpose_key(24));
        assert!(synth.keyboard_key_off(0));
        assert_eq!(synth.allocator_gate_mask(), 0);
        assert!(!synth.keyboard_key_on(KEYBOARD_KEY_COUNT, 100));
        assert!(!synth.keyboard_key_off(KEYBOARD_KEY_COUNT));
    }

    #[test]
    fn production_engine_is_six_voice_and_does_not_steal_a_seventh_note() {
        let mut synth = Synth::default();
        for note in 60..66 {
            synth.note_on(note, 100);
        }
        assert_eq!(synth.voices.len(), VOICE_COUNT);
        let notes = core::array::from_fn::<_, VOICE_COUNT, _>(|voice| synth.voices[voice].note);
        synth.note_on(66, 100);
        assert_eq!(
            core::array::from_fn::<_, VOICE_COUNT, _>(|voice| synth.voices[voice].note),
            notes
        );
        assert!(synth.voices.iter().all(|voice| voice.gate));
    }

    #[test]
    fn allocation_mode_changes_reach_physical_dsp_voices() {
        let mut synth = Synth::default();
        assert!(synth.set_parameter(
            PARAM_ALLOCATION_MODE as u32,
            AllocationMode::Unison.native_parameter()
        ));
        synth.note_on(60, 1);
        assert!(
            synth
                .voices
                .iter()
                .all(|voice| voice.gate && voice.note == 60 && voice.velocity == 1.0)
        );
        synth.note_on(48, 127);
        assert!(
            synth
                .voices
                .iter()
                .all(|voice| voice.gate && voice.note == 48 && voice.velocity == 1.0)
        );
    }

    #[test]
    fn gate_on_preserves_the_continuously_biased_filter_cell() {
        let mut synth = Synth::default();
        let _ = synth.voices[0].analog_cell.process_vcf(
            1.0,
            48_000.0,
            rf_106_voice::dac_word(rf_106_voice::VCF_CALIBRATION_WORD),
            0,
        );
        let state = synth.voices[0].analog_cell.filter_state();
        let phase = synth.voices[0].dco.phase();
        assert!(state.iter().any(|sample| *sample != 0.0));
        synth.gate_voice_on(0, 60, 100);
        assert_eq!(synth.voices[0].analog_cell.filter_state(), state);
        assert_eq!(synth.voices[0].dco.phase(), phase);
    }

    #[test]
    fn dcos_and_filter_cells_free_run_while_their_vcas_are_closed() {
        let mut synth = Synth::default();
        assert!(synth.prepare(48_000.0));
        assert!(synth.voices.iter().all(|voice| !voice.active));
        let before = core::array::from_fn::<_, VOICE_COUNT, _>(|voice| {
            (
                synth.voices[voice].dco.phase(),
                synth.voices[voice].analog_cell.filter_state(),
            )
        });
        let output = synth.process_circuit_sample();
        assert_eq!(output, (0.0, 0.0));
        for (voice, previous) in before.iter().enumerate() {
            assert_ne!(synth.voices[voice].dco.phase(), previous.0);
            assert_ne!(synth.voices[voice].analog_cell.filter_state(), previous.1);
        }
    }

    #[test]
    fn power_off_silences_and_suspends_the_circuit_until_power_on() {
        let mut synth = Synth::default();
        assert!(synth.prepare(48_000.0));
        let phases_before =
            core::array::from_fn::<_, VOICE_COUNT, _>(|voice| synth.voices[voice].dco.phase());

        assert!(synth.set_parameter(PARAM_POWER as u32, 0.0));
        for _ in 0..32 {
            assert_eq!(synth.process_circuit_sample(), (0.0, 0.0));
        }
        let phases_while_off =
            core::array::from_fn::<_, VOICE_COUNT, _>(|voice| synth.voices[voice].dco.phase());
        assert_eq!(phases_while_off, phases_before);

        assert!(synth.set_parameter(PARAM_POWER as u32, 1.0));
        let _ = synth.process_circuit_sample();
        let phases_after_power_on =
            core::array::from_fn::<_, VOICE_COUNT, _>(|voice| synth.voices[voice].dco.phase());
        assert_ne!(phases_after_power_on, phases_while_off);
    }

    #[test]
    fn physical_voice_cells_start_with_distinct_reproducible_noise_phases() {
        let mut first = Synth::default();
        let mut second = Synth::default();
        let outputs_a = core::array::from_fn::<_, VOICE_COUNT, _>(|voice| {
            first.voices[voice].analog_cell.process_vcf(
                0.0,
                48_000.0,
                rf_106_voice::dac_word(rf_106_voice::VCF_CALIBRATION_WORD),
                rf_106_voice::DAC_MAX,
            )
        });
        let outputs_b = core::array::from_fn::<_, VOICE_COUNT, _>(|voice| {
            second.voices[voice].analog_cell.process_vcf(
                0.0,
                48_000.0,
                rf_106_voice::dac_word(rf_106_voice::VCF_CALIBRATION_WORD),
                rf_106_voice::DAC_MAX,
            )
        });
        assert_eq!(outputs_a, outputs_b);
        assert!(outputs_a.windows(2).all(|pair| pair[0] != pair[1]));
    }

    #[test]
    fn hold_uses_firmware_nonzero_cc64_rule_and_preserves_release_tail() {
        let mut synth = Synth::default();
        synth.note_on(60, 100);
        synth.control_change(64, 1);
        synth.note_off(60);
        assert!(!synth.voices[0].gate);
        assert!(synth.voices[0].sustained);
        synth.control_change(64, 0);
        assert!(!synth.voices[0].sustained);
        assert!(synth.voices[0].envelope.stage() == EnvelopeStage::Release);
    }

    #[test]
    fn modern_panic_ccs_are_inert_inside_the_synth() {
        let mut synth = Synth::default();
        synth.note_on(60, 100);
        synth.note_on(64, 100);
        let gate_mask = synth.allocator.gate_mask();
        synth.control_change(120, 0);
        synth.control_change(123, 0);
        assert_eq!(synth.allocator.gate_mask(), gate_mask);
        assert!(synth.voices[0].gate && synth.voices[1].gate);
    }

    #[test]
    fn midi_channel_selection_releases_hold_notes_and_neutralizes_midi_controls() {
        let mut synth = Synth::default();
        synth.note_on(60, 100);
        synth.control_change(64, 127);
        synth.note_off(60);
        synth.control_change(1, 127);
        synth.set_pitch_bend(0x3fff);
        assert!(synth.voices[0].sustained);
        assert_ne!(synth.performance.midi_mod_depth, 0);
        assert_ne!(synth.performance.bend.amount, 0);

        synth.reset_for_midi_channel_selection();
        assert!(!synth.sustain);
        assert!(!synth.voices[0].sustained);
        assert_eq!(synth.performance.midi_mod_depth, 0);
        assert_eq!(synth.performance.bend.amount, 0);
    }

    #[test]
    fn lfo_rearm_follows_voice_running_including_hold() {
        let mut synth = Synth::default();
        assert!(synth.prepare(8_000.0));
        assert!(synth.set_parameter(PARAM_LFO_DELAY as u32, 0.0));
        synth.note_on(60, 100);
        for _ in 0..100 {
            synth.process_circuit_sample();
        }
        assert_eq!(synth.lfo.amplitude(), 255);

        synth.control_change(64, 127);
        synth.note_off(60);
        assert!(synth.voices[0].sustained);
        synth.note_on(64, 100);
        synth.process_circuit_sample();
        assert_eq!(synth.lfo.amplitude(), 255);

        synth.note_off(64);
        synth.control_change(64, 0);
        synth.process_circuit_sample();
        synth.note_on(67, 100);
        synth.process_circuit_sample();
        assert_eq!(synth.lfo.amplitude(), 0);
    }

    #[test]
    fn cc1_is_immediate_while_the_physical_lfo_trigger_declicks() {
        let mut synth = Synth::default();
        assert!(synth.prepare(8_000.0));
        assert!(synth.set_parameter(PARAM_BENDER_LFO as u32, 1.0));

        synth.control_change(1, 64);
        assert_eq!(synth.performance.process_lfo_depth(1.0), 127);
        assert_eq!(synth.performance.lfo_ramp, 0);

        synth.control_change(1, 0);
        synth.set_lfo_trigger(true);
        for _ in 0..40 {
            synth.performance.process_lfo_depth(1.0);
        }
        assert_eq!(synth.performance.lfo_ramp, 10);
        synth.set_lfo_trigger(false);
        assert_eq!(synth.performance.process_lfo_depth(1.0), 0);
    }
}
