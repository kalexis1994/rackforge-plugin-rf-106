#![no_std]

/// Mean B_2 foreground-loop period from the executed timed oracle. All held
/// control generators use this cadence; hardware jitter remains a measurement
/// input rather than being invented in the audio engine.
pub const CONTROL_TICK_SECONDS: f32 = 0.004_246_18;
pub const VOICE_COUNT: usize = 6;
pub const KEYBOARD_KEY_COUNT: u8 = 61;
const MIDI_NOTE_COUNT: usize = 128;
const MAX_ACTIONS: usize = VOICE_COUNT * 2;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum AllocationMode {
    #[default]
    Poly1 = 0,
    Poly2 = 1,
    Unison = 2,
}

impl AllocationMode {
    /// Converts the retained RF-106 native slot: 0=Unison, 1=Poly 1,
    /// 2=Poly 2. The slot remains for state compatibility, but the behavior is
    /// supplied by the firmware-derived allocator.
    pub const fn from_native_parameter(value: f64) -> Self {
        if value < 0.5 {
            Self::Unison
        } else if value < 1.5 {
            Self::Poly1
        } else {
            Self::Poly2
        }
    }

    pub const fn native_parameter(self) -> f64 {
        match self {
            Self::Unison => 0.0,
            Self::Poly1 => 1.0,
            Self::Poly2 => 2.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum NoteSource {
    Midi = 1,
    Keyboard = 2,
}

/// Firmware-derived state behind the Key Transpose button and keyboard
/// gesture. A_5 stores `offset + 12` at FFBE and applies it only to the
/// physical 61-key matrix; MIDI IN deliberately bypasses this type.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KeyTranspose {
    offset: i8,
}

impl KeyTranspose {
    pub const MIN_OFFSET: i8 = -12;
    pub const MAX_OFFSET: i8 = 12;
    pub const MIDDLE_C_KEY: u8 = 24;
    pub const LOWEST_VOICE_NOTE: u8 = 36;

    pub const fn new() -> Self {
        Self { offset: 0 }
    }

    pub const fn from_offset(offset: i8) -> Option<Self> {
        if offset >= Self::MIN_OFFSET && offset <= Self::MAX_OFFSET {
            Some(Self { offset })
        } else {
            None
        }
    }

    /// Replays the button-held key selection. Octave identity is folded on
    /// either side of middle C, preserving the sign and the +/-12 C endpoints.
    pub fn select_key(&mut self, key_index: u8) -> bool {
        let Some(offset) = Self::offset_for_key(key_index) else {
            return false;
        };
        self.offset = offset;
        true
    }

    pub const fn offset_for_key(key_index: u8) -> Option<i8> {
        if key_index >= KEYBOARD_KEY_COUNT {
            return None;
        }
        if key_index < Self::MIDDLE_C_KEY {
            Some((key_index % 12) as i8 - 12)
        } else if key_index == Self::MIDDLE_C_KEY {
            Some(0)
        } else {
            Some(((key_index - Self::MIDDLE_C_KEY - 1) % 12) as i8 + 1)
        }
    }

    pub const fn offset(self) -> i8 {
        self.offset
    }

    pub const fn stored_code(self) -> u8 {
        (self.offset + 12) as u8
    }

    pub const fn indicator(self) -> bool {
        self.offset != 0
    }

    /// Converts a physical key index into the note sent to the Voice CPU.
    pub const fn voice_note(self, key_index: u8) -> Option<u8> {
        if key_index >= KEYBOARD_KEY_COUNT {
            return None;
        }
        Some((Self::LOWEST_VOICE_NOTE as i16 + key_index as i16 + self.offset as i16) as u8)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum MidiFunction {
    One = 1,
    Two = 2,
    #[default]
    Three = 3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MidiReceiveAction {
    NoteOn { note: u8, velocity: u8 },
    NoteOff { note: u8 },
    Hold { enabled: bool },
    Modulation { value: u8 },
    PitchBend { value: u16 },
    AllNotesOff,
}

/// Message-level reconstruction of the A_5 MIDI receive policy.
///
/// RackForge currently supplies complete messages rather than serial bytes, so
/// running-status cancellation and long SysEx parsing deliberately remain
/// outside this type. Channel, Function and controller behavior are exact for
/// the retained execution fixtures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MidiReceiver {
    basic_channel: u8,
    omni: bool,
    function: MidiFunction,
}

impl Default for MidiReceiver {
    fn default() -> Self {
        Self::new()
    }
}

impl MidiReceiver {
    pub const fn new() -> Self {
        Self {
            basic_channel: 0,
            omni: true,
            function: MidiFunction::Three,
        }
    }

    /// A_5 powers up in Omni On while retaining the selected basic channel and
    /// rear-panel Function position.
    pub fn power_on(&mut self) {
        self.omni = true;
    }

    pub fn set_basic_channel(&mut self, zero_based_channel: u8) -> bool {
        if zero_based_channel > 15 {
            return false;
        }
        self.basic_channel = zero_based_channel;
        self.omni = false;
        true
    }

    pub fn set_function(&mut self, function: MidiFunction) {
        self.function = function;
    }

    pub const fn basic_channel(&self) -> u8 {
        self.basic_channel
    }

    pub const fn is_omni(&self) -> bool {
        self.omni
    }

    pub const fn function(&self) -> MidiFunction {
        self.function
    }

    pub fn receive(
        &mut self,
        status: u8,
        data_1: u8,
        data_2: u8,
        length: u8,
    ) -> Option<MidiReceiveAction> {
        if !(0x80..0xf0).contains(&status) {
            return None;
        }
        let kind = status & 0xf0;
        let channel = status & 0x0f;
        let accepted = self.omni || channel == self.basic_channel;
        let data_1 = data_1 & 0x7f;
        let data_2 = data_2 & 0x7f;
        match (kind, length) {
            (0x80, 3) if accepted => Some(MidiReceiveAction::NoteOff { note: data_1 }),
            (0x90, 3) if accepted && data_2 == 0 => {
                Some(MidiReceiveAction::NoteOff { note: data_1 })
            }
            (0x90, 3) if accepted => Some(MidiReceiveAction::NoteOn {
                note: data_1,
                velocity: data_2,
            }),
            (0xb0, 3) => self.receive_control_change(channel, data_1, data_2, accepted),
            (0xe0, 3) if accepted && self.function != MidiFunction::One => {
                Some(MidiReceiveAction::PitchBend {
                    value: u16::from(data_1) | (u16::from(data_2) << 7),
                })
            }
            _ => None,
        }
    }

    fn receive_control_change(
        &mut self,
        channel: u8,
        controller: u8,
        value: u8,
        accepted: bool,
    ) -> Option<MidiReceiveAction> {
        match controller {
            1 if accepted && self.function != MidiFunction::One => {
                Some(MidiReceiveAction::Modulation { value })
            }
            64 if accepted => Some(MidiReceiveAction::Hold {
                enabled: value != 0,
            }),
            // A_5 ignores the modern panic/reset/local-control quartet.
            120..=123 => None,
            // Channel-mode messages are recognized only on the basic channel,
            // including while Omni is active.
            124 if channel == self.basic_channel => {
                self.omni = false;
                Some(MidiReceiveAction::AllNotesOff)
            }
            125 if channel == self.basic_channel => {
                self.omni = true;
                Some(MidiReceiveAction::AllNotesOff)
            }
            126..=127 if channel == self.basic_channel => Some(MidiReceiveAction::AllNotesOff),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VoiceAction {
    GateOn {
        voice: usize,
        note: u8,
        velocity: u8,
    },
    GateOff {
        voice: usize,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActionBatch {
    actions: [Option<VoiceAction>; MAX_ACTIONS],
    len: usize,
}

impl ActionBatch {
    const fn new() -> Self {
        Self {
            actions: [None; MAX_ACTIONS],
            len: 0,
        }
    }

    fn push(&mut self, action: VoiceAction) {
        if self.len < MAX_ACTIONS {
            self.actions[self.len] = Some(action);
            self.len += 1;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = VoiceAction> + '_ {
        self.actions[..self.len].iter().copied().flatten()
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Assignment {
    note: u8,
    assigned: bool,
    gate: bool,
}

impl Assignment {
    const EMPTY: Self = Self {
        note: 60,
        assigned: false,
        gate: false,
    };
}

/// Firmware-derived six-voice assigner.
///
/// `gate` belongs to the Assigner CPU view. A DSP voice can continue running
/// after `GateOff` while Hold or its release envelope is active.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VoiceAllocator {
    mode: AllocationMode,
    voices: [Assignment; VOICE_COUNT],
    order: [usize; VOICE_COUNT],
    sources: [u8; MIDI_NOTE_COUNT],
    velocities: [u8; MIDI_NOTE_COUNT],
    unison_note: Option<u8>,
}

impl VoiceAllocator {
    pub const fn new() -> Self {
        Self {
            mode: AllocationMode::Poly1,
            voices: [Assignment::EMPTY; VOICE_COUNT],
            order: [0, 1, 2, 3, 4, 5],
            sources: [0; MIDI_NOTE_COUNT],
            velocities: [0; MIDI_NOTE_COUNT],
            unison_note: None,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub const fn mode(&self) -> AllocationMode {
        self.mode
    }

    pub fn voice_notes(&self) -> [u8; VOICE_COUNT] {
        core::array::from_fn(|voice| self.voices[voice].note)
    }

    pub fn gate_mask(&self) -> u8 {
        self.voices
            .iter()
            .enumerate()
            .fold(0, |mask, (voice, state)| {
                mask | (u8::from(state.gate) << voice)
            })
    }

    pub fn set_mode(&mut self, mode: AllocationMode) -> ActionBatch {
        let mut actions = ActionBatch::new();
        if mode == self.mode {
            return actions;
        }
        let previous = self.mode;
        self.mode = mode;

        match (previous, mode) {
            (AllocationMode::Poly1, AllocationMode::Poly2)
            | (AllocationMode::Poly2, AllocationMode::Poly1) => {
                for assignment in &mut self.voices {
                    if !assignment.gate {
                        assignment.assigned = false;
                    }
                }
                self.order = [0, 1, 2, 3, 4, 5];
            }
            (_, AllocationMode::Unison) => {
                for assignment in &mut self.voices {
                    assignment.assigned = false;
                }
                if let Some(note) = self.highest_held_note() {
                    self.unison_note = Some(note);
                    self.gate_unison(note, self.velocities[note as usize], &mut actions);
                } else {
                    self.unison_note = None;
                    self.gate_off_all(&mut actions);
                }
            }
            (AllocationMode::Unison, _) => {
                self.unison_note = None;
                self.gate_off_all(&mut actions);
                for assignment in &mut self.voices {
                    assignment.assigned = false;
                }
                self.order = [0, 1, 2, 3, 4, 5];
                for note in (0..MIDI_NOTE_COUNT).rev() {
                    if self.sources[note] != 0 {
                        self.allocate_poly(note as u8, self.velocities[note], &mut actions);
                    }
                }
            }
            _ => {}
        }
        actions
    }

    pub fn note_on(&mut self, source: NoteSource, note: u8, velocity: u8) -> ActionBatch {
        if velocity == 0 {
            return self.note_off(source, note);
        }
        let note = fold_midi_note(note);
        let source = source as u8;
        let sources = &mut self.sources[note as usize];
        if *sources & source != 0 {
            return ActionBatch::new();
        }
        let was_held = *sources != 0;
        *sources |= source;
        self.velocities[note as usize] = velocity.min(127);
        if was_held {
            return ActionBatch::new();
        }

        let mut actions = ActionBatch::new();
        match self.mode {
            AllocationMode::Poly1 | AllocationMode::Poly2 => {
                self.allocate_poly(note, velocity.min(127), &mut actions);
            }
            AllocationMode::Unison => {
                self.unison_note = Some(note);
                self.gate_unison(note, velocity.min(127), &mut actions);
            }
        }
        actions
    }

    pub fn note_off(&mut self, source: NoteSource, note: u8) -> ActionBatch {
        let note = fold_midi_note(note);
        let source = source as u8;
        let sources = &mut self.sources[note as usize];
        if *sources & source == 0 {
            return ActionBatch::new();
        }
        *sources &= !source;
        if *sources != 0 {
            return ActionBatch::new();
        }

        let mut actions = ActionBatch::new();
        match self.mode {
            AllocationMode::Poly1 | AllocationMode::Poly2 => {
                if let Some(voice) = self
                    .voices
                    .iter()
                    .position(|assignment| assignment.gate && assignment.note == note)
                {
                    self.voices[voice].gate = false;
                    actions.push(VoiceAction::GateOff { voice });
                    if self.mode == AllocationMode::Poly1 {
                        self.move_order_to_end(voice);
                    }
                }
            }
            AllocationMode::Unison if self.unison_note == Some(note) => {
                if let Some(fallback) = self.highest_held_note() {
                    self.unison_note = Some(fallback);
                    self.gate_unison(fallback, self.velocities[fallback as usize], &mut actions);
                } else {
                    self.unison_note = None;
                    self.gate_off_all(&mut actions);
                }
            }
            AllocationMode::Unison => {}
        }
        actions
    }

    pub fn all_notes_off(&mut self) -> ActionBatch {
        self.sources.fill(0);
        self.velocities.fill(0);
        self.unison_note = None;
        let mut actions = ActionBatch::new();
        self.gate_off_all(&mut actions);
        actions
    }

    fn allocate_poly(&mut self, note: u8, velocity: u8, actions: &mut ActionBatch) {
        let recalled = if self.mode == AllocationMode::Poly1 {
            self.voices.iter().position(|assignment| {
                assignment.assigned && !assignment.gate && assignment.note == note
            })
        } else {
            None
        };
        let voice = recalled.or_else(|| {
            self.order
                .iter()
                .copied()
                .find(|voice| !self.voices[*voice].gate)
        });
        let Some(voice) = voice else {
            return;
        };
        self.voices[voice] = Assignment {
            note,
            assigned: true,
            gate: true,
        };
        if recalled.is_some() {
            self.move_order_to_front(voice);
        }
        actions.push(VoiceAction::GateOn {
            voice,
            note,
            velocity,
        });
    }

    fn gate_unison(&mut self, note: u8, velocity: u8, actions: &mut ActionBatch) {
        for (voice, assignment) in self.voices.iter_mut().enumerate() {
            assignment.note = note;
            assignment.gate = true;
            assignment.assigned = false;
            actions.push(VoiceAction::GateOn {
                voice,
                note,
                velocity,
            });
        }
    }

    fn gate_off_all(&mut self, actions: &mut ActionBatch) {
        for (voice, assignment) in self.voices.iter_mut().enumerate() {
            if assignment.gate {
                assignment.gate = false;
                actions.push(VoiceAction::GateOff { voice });
            }
        }
    }

    fn highest_held_note(&self) -> Option<u8> {
        self.sources
            .iter()
            .rposition(|sources| *sources != 0)
            .map(|note| note as u8)
    }

    fn move_order_to_end(&mut self, voice: usize) {
        let Some(position) = self.order.iter().position(|candidate| *candidate == voice) else {
            return;
        };
        self.order[position..].rotate_left(1);
    }

    fn move_order_to_front(&mut self, voice: usize) {
        let Some(position) = self.order.iter().position(|candidate| *candidate == voice) else {
            return;
        };
        self.order[..=position].rotate_right(1);
    }
}

impl Default for VoiceAllocator {
    fn default() -> Self {
        Self::new()
    }
}

/// The Assigner folds out-of-keyboard MIDI pitches by octaves before voice
/// allocation. The resulting Voice CPU range is 24..=108.
pub const fn fold_midi_note(note: u8) -> u8 {
    match note {
        0..=11 => note + 24,
        12..=23 => note + 12,
        24..=108 => note,
        109..=120 => note - 12,
        _ => note - 24,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BendState {
    pub negative: bool,
    pub amount: u8,
}

impl BendState {
    pub const CENTER: Self = Self {
        negative: false,
        amount: 0,
    };

    pub fn from_midi(value: u16) -> Self {
        let value = value.min(16_383);
        let effective = (((value >> 7) * 2) + ((value & 0x7f) >> 6)) as u8;
        let negative = effective < 128;
        let magnitude = if negative {
            127 - effective
        } else {
            effective - 128
        };
        Self {
            negative,
            amount: if magnitude == 0 {
                0
            } else {
                magnitude.saturating_mul(2).saturating_add(1)
            },
        }
    }

    pub fn unit(self) -> f32 {
        let amount = f32::from(self.amount) / 255.0;
        if self.negative { -amount } else { amount }
    }

    pub fn dco_semitones(self, sensitivity: f32) -> f32 {
        let sensitivity = sensitivity.clamp(0.0, 1.0);
        let product = (u16::from(self.amount) * libm::roundf(sensitivity * 255.0) as u16) as u32;
        let fixed = (product >> 1) + (product >> 2) + (product >> 8);
        let signed = fixed as f32 / 4_096.0;
        if self.negative { -signed } else { signed }
    }

    /// Unsigned bend offset consumed directly by the Voice CPU's 14-bit VCF
    /// accumulator. Full-scale sensitivity and bend produce `0x0fe0`.
    pub fn vcf_offset_word(self, sensitivity: f32) -> u16 {
        let sensitivity = libm::roundf(sensitivity.clamp(0.0, 1.0) * 255.0) as u16;
        ((sensitivity * u16::from(self.amount)) >> 4).min(4_095)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PerformanceState {
    pub bend: BendState,
    pub midi_mod_depth: u8,
    pub lfo_ramp: u8,
    pub lfo_trigger_active: bool,
    control_tick_samples: u32,
    samples_until_control_tick: u32,
}

impl PerformanceState {
    pub const fn new() -> Self {
        Self {
            bend: BendState::CENTER,
            midi_mod_depth: 0,
            lfo_ramp: 0,
            lfo_trigger_active: false,
            control_tick_samples: 205,
            samples_until_control_tick: 205,
        }
    }

    pub fn prepare(&mut self, sample_rate: f32) {
        self.control_tick_samples =
            libm::roundf(sample_rate * CONTROL_TICK_SECONDS).max(1.0) as u32;
        self.reset();
    }

    pub fn reset(&mut self) {
        self.bend = BendState::CENTER;
        self.midi_mod_depth = 0;
        self.lfo_ramp = 0;
        self.lfo_trigger_active = false;
        self.samples_until_control_tick = self.control_tick_samples;
    }

    pub fn set_pitch_bend(&mut self, value: u16) {
        self.bend = BendState::from_midi(value);
    }

    pub fn set_modulation(&mut self, value: u8) {
        self.midi_mod_depth = value.min(127).saturating_mul(2);
    }

    /// Sets the semantic state of the active-low physical LFO Trigger input.
    /// ADC thresholding and hysteresis belong to the board/input adapter.
    pub fn set_lfo_trigger(&mut self, active: bool) {
        self.lfo_trigger_active = active;
        if !active {
            self.lfo_ramp = 0;
        }
    }

    pub fn process_lfo_depth(&mut self, sensitivity: f32) -> u8 {
        let sensitivity = libm::roundf(sensitivity.clamp(0.0, 1.0) * 255.0) as u8;
        let midi = scale_performance_lfo_depth(self.midi_mod_depth, sensitivity);
        if !self.lfo_trigger_active {
            self.lfo_ramp = 0;
        }
        self.samples_until_control_tick = self.samples_until_control_tick.saturating_sub(1);
        if self.samples_until_control_tick == 0 {
            self.samples_until_control_tick = self.control_tick_samples;
            // B_2 tests before adding, so the last ten-count step may cross
            // sensitivity. The effective contribution is clamped afterwards.
            if self.lfo_trigger_active && self.lfo_ramp < sensitivity {
                self.lfo_ramp = self.lfo_ramp.saturating_add(10);
            }
        }
        midi.max(self.lfo_ramp.min(sensitivity))
    }
}

impl Default for PerformanceState {
    fn default() -> Self {
        Self::new()
    }
}

/// Compact reconstruction of B_2's 128-entry LFO speed law. The regular table
/// regions are represented algebraically and the few accelerating boundaries
/// are expressed as short segments; no ROM image is embedded in the plugin.
pub const fn lfo_speed_coefficient(raw: u8) -> u16 {
    let raw = if raw > 127 { 127 } else { raw };
    match raw {
        0 => 5,
        1 => 15,
        2 => 25,
        3..=5 => 40 + (raw - 3) as u16 * 15,
        6..=7 => 80 + (raw - 6) as u16 * 10,
        8..=63 => raw as u16 * 10 + 20,
        64..=95 => 666 + (raw - 64) as u16 * 16,
        96..=102 => 1_214 + (raw - 96) as u16 * 52,
        103 => 1_580,
        104..=105 => 1_650 + (raw - 104) as u16 * 70,
        106..=109 => 1_800 + (raw - 106) as u16 * 80,
        110..=111 => 2_140 + (raw - 110) as u16 * 100,
        112..=119 => 2_340 + (raw - 112) as u16 * 100,
        120..=122 => 3_160 + (raw - 120) as u16 * 120,
        123..=126 => 3_550 + (raw - 123) as u16 * 150,
        _ => 4_096,
    }
}

pub const fn lfo_rate_hz(raw: u8) -> f32 {
    let coefficient = lfo_speed_coefficient(raw) as u32;
    let ticks_per_half_sweep = 8_192_u32.div_ceil(coefficient);
    1.0 / (4.0 * ticks_per_half_sweep as f32 * CONTROL_TICK_SECONDS)
}

/// B_2 indexes this ramp law with the upper three bits of LFO Delay.
pub const fn lfo_delay_ramp_increment(raw: u8) -> u16 {
    match (if raw > 127 { 127 } else { raw }) >> 4 {
        0 => 0xffff,
        1 => 0x0419,
        2 => 0x020c,
        3 => 0x015e,
        _ => 0x0100,
    }
}

/// B_2's compact nonlinear transfer from the DCO LFO panel value to the
/// 8-bit multiplier used by the retained destination calculation.
pub const fn dco_lfo_depth(raw: u8) -> u8 {
    let raw = if raw > 127 { 127 } else { raw };
    match raw {
        0..=2 => 0,
        3..=64 => raw - 2,
        65..=95 => 62 + (raw - 64) * 2,
        96..=124 => 128 + (raw - 96) * 4,
        125 => 248,
        _ => 255,
    }
}

/// Unsigned magnitude in the DCO's 8.8 pitch-word domain. At full depth the
/// executed B_2 path reaches 0x03f7, just below four semitones.
pub const fn dco_lfo_destination(accumulator: u16, amplitude: u8, raw: u8) -> u16 {
    dco_lfo_destination_from_depth(accumulator, amplitude, dco_lfo_depth(raw))
}

pub const fn dco_lfo_destination_from_depth(accumulator: u16, amplitude: u8, depth: u8) -> u16 {
    let scaled_depth = ((depth as u16) * (amplitude as u16)) >> 8;
    dco_lfo_destination_from_coefficient(accumulator, scaled_depth as u8)
}

/// Scales MIDI CC1's doubled byte by the physical LFO sensitivity control.
pub const fn scale_performance_lfo_depth(depth: u8, sensitivity: u8) -> u8 {
    ((depth as u16 * sensitivity as u16) >> 8) as u8
}

/// Combines the delayed patch depth with the already sensitivity-scaled
/// performance path. B_2 saturates this sum before applying LFO phase.
pub const fn combine_dco_lfo_depth(raw: u8, amplitude: u8, performance: u8) -> u8 {
    let panel = ((dco_lfo_depth(raw) as u16 * amplitude as u16) >> 8) as u8;
    panel.saturating_add(performance)
}

/// Applies an already composed 8-bit DCO LFO coefficient to retained phase.
pub const fn dco_lfo_destination_from_coefficient(accumulator: u16, coefficient: u8) -> u16 {
    let accumulator = if accumulator > 0x1fff {
        0x1fff
    } else {
        accumulator
    };
    ((accumulator as u32 * coefficient as u32) >> 11) as u16
}

/// Unsigned magnitude in the 14-bit VCF DAC domain. The panel control is
/// doubled before B_2 applies delay amplitude and accumulator scaling.
pub const fn vcf_lfo_destination(accumulator: u16, amplitude: u8, raw: u8) -> u16 {
    let raw = if raw > 127 { 127 } else { raw };
    let accumulator = if accumulator > 0x1fff {
        0x1fff
    } else {
        accumulator
    };
    let depth = raw * 2;
    let scaled_depth = ((depth as u16) * (amplitude as u16)) >> 8;
    ((accumulator as u32 * scaled_depth as u32) >> 9) as u16
}

/// Retained bipolar LFO phase as the unsigned 14-bit source consumed by B_2's
/// PWM path. Unlike DCO/VCF modulation, PWM does not apply LFO Delay amplitude.
pub const fn pwm_lfo_source(accumulator: u16, negative: bool) -> u16 {
    let accumulator = if accumulator > 0x1fff {
        0x1fff
    } else {
        accumulator
    };
    if negative {
        0x2000 - accumulator
    } else {
        0x2000 + accumulator
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GlobalLfo {
    accumulator: u16,
    rising: bool,
    negative: bool,
    holdoff_accumulator: u16,
    ramp_accumulator: u16,
    in_holdoff: bool,
    armed: bool,
    was_gated: bool,
    amplitude: u8,
    held_output: f32,
    control_tick_samples: u32,
    samples_until_control_tick: u32,
}

impl GlobalLfo {
    pub const fn new() -> Self {
        Self {
            accumulator: 0,
            rising: true,
            negative: false,
            holdoff_accumulator: 0,
            ramp_accumulator: 0xffff,
            in_holdoff: false,
            armed: true,
            was_gated: false,
            amplitude: 0xff,
            held_output: 0.0,
            control_tick_samples: 204,
            samples_until_control_tick: 0,
        }
    }

    pub fn prepare(&mut self, sample_rate: f32) {
        self.control_tick_samples =
            libm::roundf(sample_rate * CONTROL_TICK_SECONDS).max(1.0) as u32;
        self.reset();
    }

    pub fn reset(&mut self) {
        let tick_samples = self.control_tick_samples;
        *self = Self::new();
        self.control_tick_samples = tick_samples.max(1);
    }

    pub fn process(&mut self, rate_control: f32, delay_control: f32, gated: bool) -> f32 {
        if !gated {
            self.armed = true;
        }
        if gated && !self.was_gated && self.armed {
            self.holdoff_accumulator = 0;
            self.ramp_accumulator = 0;
            self.amplitude = 0;
            self.in_holdoff = true;
            self.armed = false;
        }
        self.was_gated = gated;

        if self.samples_until_control_tick == 0 {
            self.tick(
                panel_control_raw(rate_control),
                panel_control_raw(delay_control),
            );
            self.samples_until_control_tick = self.control_tick_samples;
        }
        self.samples_until_control_tick -= 1;
        self.held_output
    }

    fn tick(&mut self, rate_raw: u8, delay_raw: u8) {
        let coefficient = lfo_speed_coefficient(rate_raw);
        if self.rising {
            let next = u32::from(self.accumulator) + u32::from(coefficient);
            if next >= 0x2000 {
                self.accumulator = 0x1fff;
                self.rising = false;
            } else {
                self.accumulator = next as u16;
            }
        } else if coefficient > self.accumulator {
            self.accumulator = 0;
            self.rising = true;
            self.negative = !self.negative;
        } else {
            self.accumulator -= coefficient;
        }

        if self.in_holdoff {
            let next = u32::from(self.holdoff_accumulator)
                + u32::from(crate::lfo_holdoff_increment(delay_raw));
            if next >= 0x4000 {
                self.holdoff_accumulator = 0x4000;
                self.in_holdoff = false;
            } else {
                self.holdoff_accumulator = next as u16;
            }
            self.amplitude = 0;
        } else if self.ramp_accumulator != 0xffff {
            let increment = lfo_delay_ramp_increment(delay_raw);
            let (next, overflow) = self.ramp_accumulator.overflowing_add(increment);
            if overflow {
                self.ramp_accumulator = 0xffff;
                self.amplitude = 0xff;
            } else {
                self.ramp_accumulator = next;
                self.amplitude = (next >> 8) as u8;
            }
        }

        let magnitude = f32::from(self.accumulator) / 8_191.0;
        let signed = if self.negative { -magnitude } else { magnitude };
        self.held_output = signed * (f32::from(self.amplitude) / 255.0);
    }

    pub const fn accumulator(&self) -> u16 {
        self.accumulator
    }

    pub const fn amplitude(&self) -> u8 {
        self.amplitude
    }

    pub const fn is_negative(&self) -> bool {
        self.negative
    }

    pub const fn dco_destination(&self, raw: u8) -> i16 {
        let magnitude = dco_lfo_destination(self.accumulator, self.amplitude, raw) as i16;
        if self.negative { -magnitude } else { magnitude }
    }

    pub const fn vcf_destination(&self, raw: u8) -> i16 {
        let magnitude = vcf_lfo_destination(self.accumulator, self.amplitude, raw) as i16;
        if self.negative { -magnitude } else { magnitude }
    }

    pub const fn pwm_source(&self) -> u16 {
        pwm_lfo_source(self.accumulator, self.negative)
    }
}

impl Default for GlobalLfo {
    fn default() -> Self {
        Self::new()
    }
}

const fn panel_control_raw(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 127.0 + 0.5) as u8
}

/// The LFO holdoff reuses the same behavioral attack law as B_2's envelope.
/// This local form avoids coupling the control crate back to the voice crate.
pub fn lfo_holdoff_increment(raw: u8) -> u16 {
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

/// Compact, byte-exact representation of B_2's 7-bit portamento coefficient
/// law. The private ROM lookup is not embedded in the plugin.
pub const fn portamento_coefficient(raw: u8) -> u8 {
    let raw = if raw > 127 { 127 } else { raw };
    match raw {
        0 => 0,
        1..=25 => 255 - 8 * (raw - 1),
        26..=47 => 61 - 2 * (raw - 26),
        48 => 18,
        49 => 17,
        50..=53 => 16,
        54..=57 => 15,
        58..=61 => 14,
        62..=66 => 13,
        67..=71 => 12,
        72..=76 => 11,
        77..=81 => 10,
        82..=86 => 9,
        87..=91 => 8,
        92..=96 => 7,
        97..=101 => 6,
        102..=106 => 5,
        107..=111 => 4,
        112..=116 => 3,
        117..=121 => 2,
        _ => 1,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Portamento {
    pitch_8_8: u16,
    samples_until_tick: u32,
}

impl Default for Portamento {
    fn default() -> Self {
        Self::new()
    }
}

impl Portamento {
    pub const fn new() -> Self {
        Self {
            // B_2 initializes the six note bytes to middle C; coefficient zero
            // then seeds all six persistent pitch words from that value.
            pitch_8_8: 60 << 8,
            samples_until_tick: 0,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Advances the B_2 pitch accumulator from the effective Bender Board
    /// voltage. Service Notes show SW1 in series with the VR2 wiper and R16
    /// pulling the Voice `PORTAMENTO` input to ground while the switch is off.
    pub fn process(
        &mut self,
        sample_rate: f32,
        target_note: u8,
        control: f32,
        switch_on: bool,
    ) -> u16 {
        let target = u16::from(target_note) << 8;
        if self.samples_until_tick == 0 {
            let effective_control = if switch_on { control } else { 0.0 };
            let panel_raw = libm::roundf(effective_control.clamp(0.0, 1.0) * 127.0) as u8;
            let coefficient = portamento_coefficient(panel_raw);
            self.pitch_8_8 = step_portamento(self.pitch_8_8, target, coefficient);
            self.samples_until_tick =
                libm::roundf(sample_rate * CONTROL_TICK_SECONDS).max(1.0) as u32;
        }
        self.samples_until_tick -= 1;
        self.pitch_8_8
    }

    pub const fn pitch_8_8(&self) -> u16 {
        self.pitch_8_8
    }
}

const fn step_portamento(current: u16, target: u16, coefficient: u8) -> u16 {
    if coefficient == 0 || current == target {
        target
    } else if current < target {
        let next = current.saturating_add(coefficient as u16);
        if next > target { target } else { next }
    } else {
        let next = current.saturating_sub(coefficient as u16);
        if next < target { target } else { next }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn midi_channel_selection_enters_omni_off_and_power_on_restores_omni() {
        let mut receiver = MidiReceiver::new();
        assert!(receiver.set_basic_channel(9));
        assert_eq!(receiver.basic_channel(), 9);
        assert!(!receiver.is_omni());
        assert_eq!(receiver.receive(0x90, 60, 100, 3), None);
        assert!(matches!(
            receiver.receive(0x99, 60, 100, 3),
            Some(MidiReceiveAction::NoteOn { .. })
        ));
        assert!(!receiver.set_basic_channel(16));
        assert_eq!(receiver.basic_channel(), 9);

        receiver.power_on();
        assert!(receiver.is_omni());
        assert_eq!(receiver.basic_channel(), 9);
        assert!(matches!(
            receiver.receive(0x90, 62, 100, 3),
            Some(MidiReceiveAction::NoteOn { .. })
        ));
    }

    #[test]
    fn portamento_coefficient_law_matches_b2_regions() {
        let anchors = [
            (0, 0),
            (1, 255),
            (25, 63),
            (26, 61),
            (47, 19),
            (48, 18),
            (49, 17),
            (50, 16),
            (53, 16),
            (54, 15),
            (61, 14),
            (62, 13),
            (121, 2),
            (122, 1),
            (127, 1),
        ];
        for (raw, expected) in anchors {
            assert_eq!(portamento_coefficient(raw), expected, "raw={raw}");
        }
    }

    #[test]
    fn portamento_moves_in_8_8_steps_and_clamps_to_target() {
        let mut portamento = Portamento::new();
        assert_eq!(portamento.process(1.0, 72, 0.0, true), 72 << 8);
        assert_eq!(portamento.process(1.0, 48, 2.0 / 127.0, true), 18_185);
        for _ in 0..30 {
            portamento.process(1.0, 48, 2.0 / 127.0, true);
        }
        assert_eq!(portamento.pitch_8_8(), 48 << 8);

        let mut slow = Portamento::new();
        slow.process(1.0, 48, 0.0, true);
        assert_eq!(slow.process(1.0, 72, 1.0, true), (48 << 8) + 1);
    }

    #[test]
    fn physical_portamento_switch_grounds_the_adc_without_moving_the_knob() {
        let mut portamento = Portamento::new();
        assert_eq!(portamento.process(1.0, 48, 0.0, true), 48 << 8);
        assert_eq!(portamento.process(1.0, 72, 1.0, true), (48 << 8) + 1);

        // SW1 opens, R16 pulls AN1 to zero and B_2's coefficient-zero path
        // snaps to the target on its next control tick.
        assert_eq!(portamento.process(1.0, 72, 1.0, false), 72 << 8);

        // Reopening SW1 preserves the knob setting but cannot recreate the old
        // glide: the persistent accumulator is already at the current note.
        assert_eq!(portamento.process(1.0, 72, 1.0, true), 72 << 8);
        assert_eq!(portamento.process(1.0, 60, 1.0, true), (72 << 8) - 1);
    }

    #[test]
    fn lfo_speed_law_matches_rom_region_boundaries() {
        let anchors = [
            (0, 5),
            (7, 90),
            (8, 100),
            (63, 650),
            (64, 666),
            (95, 1_162),
            (96, 1_214),
            (103, 1_580),
            (111, 2_240),
            (119, 3_040),
            (125, 3_850),
            (126, 4_000),
            (127, 4_096),
        ];
        for (raw, coefficient) in anchors {
            assert_eq!(lfo_speed_coefficient(raw), coefficient, "raw {raw}");
        }
        assert!((lfo_rate_hz(0) - 0.035_94).abs() < 0.0001);
        assert!((lfo_rate_hz(127) - 29.438).abs() < 0.01);
    }

    #[test]
    fn lfo_accumulator_clamps_and_changes_polarity_every_second_turn() {
        let mut lfo = GlobalLfo::new();
        lfo.control_tick_samples = 1;
        let mut states = [(0, false); 8];
        for state in &mut states {
            lfo.process(1.0, 0.0, false);
            *state = (lfo.accumulator(), lfo.is_negative());
        }
        assert_eq!(
            states,
            [
                (4096, false),
                (8191, false),
                (4095, false),
                (0, true),
                (4096, true),
                (8191, true),
                (4095, true),
                (0, false),
            ]
        );
    }

    #[test]
    fn lfo_delay_is_holdoff_then_eight_bit_linear_ramp() {
        let mut lfo = GlobalLfo::new();
        lfo.control_tick_samples = 1;
        lfo.process(0.0, 0.0, true);
        assert_eq!(lfo.amplitude(), 0);
        lfo.process(0.0, 0.0, true);
        assert_eq!(lfo.amplitude(), 255);

        let mut slow = GlobalLfo::new();
        slow.control_tick_samples = 1;
        for _ in 0..781 {
            slow.process(0.0, 1.0, true);
            assert_eq!(slow.amplitude(), 0);
        }
        slow.process(0.0, 1.0, true);
        assert_eq!(slow.amplitude(), 1);
    }

    #[test]
    fn lfo_delay_rearms_only_after_all_gates_clear() {
        let mut lfo = GlobalLfo::new();
        lfo.control_tick_samples = 1;
        lfo.process(0.0, 0.0, true);
        lfo.process(0.0, 0.0, true);
        assert_eq!(lfo.amplitude(), 255);
        lfo.process(0.0, 0.0, false);
        lfo.process(0.0, 0.0, true);
        assert_eq!(lfo.amplitude(), 0);
    }
}
