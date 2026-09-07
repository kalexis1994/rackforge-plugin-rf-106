extern crate alloc;

use core::{mem, slice};

mod catalog;
mod j106_library;
mod program;
mod programs;

use alloc::{format, string::String, string::ToString, vec::Vec};
use program::{
    PROGRAM_EDIT_SCHEMA_VERSION, PROGRAM_SCHEMA_VERSION, PreparedProgram, ProgramDocument,
    ProgramEditRequest, ProgramFieldEditRequest,
};
use programs::{CASSETTE_BAYS, CustomPrograms, Program, cassette_bay, cassette_program};
use rackforge_plugin_sdk::{
    BlockContext, MidiEvent, PROGRAM_EDIT_BASIC, PROGRAM_EDIT_DECLARATIVE, PROGRAM_EDIT_PREVIEW,
    ParameterEvent, PlanWriter, Processor, UnitContext, UnitMix, export_parallel_processor,
};
use rf_106_contract::{
    DEFAULT_FACTORY_PROGRAM, NATIVE_PARAMETER_COUNT, PUBLIC_BENDER_POSITION_INDEX,
    PUBLIC_CHORUS_MODE_INDEX, PUBLIC_KEY_TRANSPOSE_TRIGGER_INDEX, PUBLIC_LFO_TRIGGER_INDEX,
    PUBLIC_MIDI_CHANNEL_INDEX, PUBLIC_MIDI_FUNCTION_INDEX, PUBLIC_PHYSICAL_VOLUME_INDEX,
    PUBLIC_PORTAMENTO_SWITCH_INDEX, STATE_SCHEMA_VERSION, factory_preset, parse_preset_id,
    public_parameter_value_is_valid, public_to_native_parameter,
    sysex::{APR_BYTES, Tone},
};
use rf_106_control::{MidiFunction, MidiReceiveAction, MidiReceiver, VOICE_COUNT};
use rf_106_dsp::{
    CommonVoiceFrame, FinishFrame, ParallelVoiceUnit, Synth, VoiceCommand, VoiceCommandKind,
};
use serde::Serialize;

const STATE_MAGIC: [u8; 4] = *b"R106";
const PARAMETER_STATE_END: usize = 12 + NATIVE_PARAMETER_COUNT * 8;
const MIDI_STATE_OFFSET: usize = PARAMETER_STATE_END;
const PORTAMENTO_STATE_OFFSET: usize = MIDI_STATE_OFFSET + 2;
const VOLUME_STATE_OFFSET: usize = PORTAMENTO_STATE_OFFSET + 1;
const STATE_BYTES: usize = VOLUME_STATE_OFFSET + 8;
const NATIVE_HOST_MASTER: u32 = 44;
const HOST_GAIN_AT_UNITY: f32 = 0.32;
const HOST_LIMIT_THRESHOLD: f32 = 0.90;
const HOST_LIMIT_CEILING: f32 = 0.98;
const MAX_RESOURCE_BYTES: usize = 1_048_576;
const MAX_FRAMES: usize = 4096;
const MAX_OUTPUT_CHANNELS: usize = 2;
const MAX_MIDI_EVENTS: usize = 1024;
const MAX_PARAMETER_EVENTS: usize = 1024;
const MAX_COMMANDS_PER_UNIT: usize = MAX_MIDI_EVENTS + MAX_PARAMETER_EVENTS + VOICE_COUNT;
const PARALLEL_WIRE_VERSION: u32 = 1;
const SHARED_MAGIC: u32 = u32::from_le_bytes(*b"R6SH");
const DISPATCH_MAGIC: u32 = u32::from_le_bytes(*b"R6DU");

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
struct SharedHeader {
    magic: u32,
    version: u32,
    frames: u32,
    sample_rate_bits: u32,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
struct DispatchHeader {
    magic: u32,
    version: u32,
    frames: u32,
    initial_epoch: u32,
    command_count: u32,
    reserved: u32,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
struct WireVoiceCommand {
    frame: u32,
    epoch: u32,
    kind: u8,
    note: u8,
    velocity: u8,
    sustain: u8,
    unit: u8,
    reserved: [u8; 3],
}

impl WireVoiceCommand {
    fn from_command(frame: u32, command: VoiceCommand) -> Self {
        Self {
            frame,
            epoch: command.epoch,
            kind: command.kind as u8,
            note: command.note,
            velocity: command.velocity,
            sustain: command.sustain,
            unit: command.unit,
            reserved: [0; 3],
        }
    }

    fn decode(self) -> Option<VoiceCommand> {
        let kind = match self.kind {
            0 => VoiceCommandKind::Reset,
            1 => VoiceCommandKind::GateOn,
            2 => VoiceCommandKind::GateOff,
            3 => VoiceCommandKind::ReleaseSustain,
            _ => return None,
        };
        Some(VoiceCommand {
            unit: self.unit,
            kind,
            note: self.note,
            velocity: self.velocity,
            sustain: self.sustain,
            reserved: [0; 3],
            epoch: self.epoch,
        })
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct EndFrame {
    finish: FinishFrame,
    host_master: f32,
}

const SHARED_HEADER_BYTES: usize = mem::size_of::<SharedHeader>();
const COMMON_FRAME_BYTES: usize = mem::size_of::<CommonVoiceFrame>();
const SHARED_CAPACITY: usize = SHARED_HEADER_BYTES + MAX_FRAMES * COMMON_FRAME_BYTES;
const DISPATCH_HEADER_BYTES: usize = mem::size_of::<DispatchHeader>();
const COMMAND_BYTES: usize = MAX_COMMANDS_PER_UNIT * mem::size_of::<WireVoiceCommand>();
const DISPATCH_STRIDE: usize = DISPATCH_HEADER_BYTES + COMMAND_BYTES;

const _: () = assert!(SHARED_CAPACITY.is_multiple_of(8));
const _: () = assert!(DISPATCH_STRIDE.is_multiple_of(8));

struct Rf106 {
    synth: Synth,
    midi: MidiReceiver,
    selected_factory_program: u32,
    key_transpose_armed: bool,
    custom: CustomPrograms,
    cassettes: [Vec<Program>; CASSETTE_BAYS],
    cassette_names: [Option<String>; CASSETTE_BAYS],
    incoming: Vec<u8>,
    receiving_cassette: Option<usize>,
    end_frames: Vec<EndFrame>,
    commands: [Vec<WireVoiceCommand>; VOICE_COUNT],
    dispatch_scratch: Vec<u8>,
    command_overflow: bool,
}

impl Default for Rf106 {
    fn default() -> Self {
        Self {
            synth: Synth::default(),
            midi: MidiReceiver::new(),
            selected_factory_program: DEFAULT_FACTORY_PROGRAM,
            key_transpose_armed: false,
            custom: CustomPrograms::default(),
            cassettes: core::array::from_fn(|_| Vec::new()),
            cassette_names: core::array::from_fn(|_| None),
            incoming: Vec::new(),
            receiving_cassette: None,
            end_frames: alloc::vec![EndFrame::default(); MAX_FRAMES],
            commands: core::array::from_fn(|_| Vec::with_capacity(MAX_COMMANDS_PER_UNIT)),
            dispatch_scratch: alloc::vec![0; DISPATCH_STRIDE],
            command_overflow: false,
        }
    }
}

impl Rf106 {
    fn capture_voice_commands(&mut self, frame: u32) {
        let mut pending = [VoiceCommand::default(); VOICE_COUNT * 8];
        let count = self.synth.drain_voice_commands(&mut pending);
        for command in pending[..count].iter().copied() {
            let unit = command.unit as usize;
            let Some(destination) = self.commands.get_mut(unit) else {
                self.command_overflow = true;
                continue;
            };
            if destination.len() >= MAX_COMMANDS_PER_UNIT {
                self.command_overflow = true;
                continue;
            }
            destination.push(WireVoiceCommand::from_command(frame, command));
        }
    }

    fn write_dispatch(&mut self, unit: usize, frames: usize, initial_epoch: u32) -> usize {
        self.dispatch_scratch.fill(0);
        let command_count = self.commands[unit].len();
        let header = DispatchHeader {
            magic: DISPATCH_MAGIC,
            version: PARALLEL_WIRE_VERSION,
            frames: frames as u32,
            initial_epoch,
            command_count: command_count as u32,
            reserved: 0,
        };
        let mut offset = 0;
        write_value(&mut self.dispatch_scratch, &mut offset, &header);
        write_values(
            &mut self.dispatch_scratch,
            &mut offset,
            &self.commands[unit],
        );
        offset
    }

    fn tone_for_catalog_id(&self, id: &str) -> Option<(Tone, Option<String>, String)> {
        if let Some(document_id) = CustomPrograms::document_id(id) {
            let program = self.custom.find(document_id)?;
            return Some((program.tone, Some(program.id.clone()), program.name.clone()));
        }
        if let Some((bay, index)) = cassette_program(id) {
            let program = self.cassettes.get(bay)?.get(index)?;
            return Some((program.tone, None, program.name.clone()));
        }
        let index = parse_preset_id(id)?;
        let preset = factory_preset(index)?;
        Some((Tone::from_factory(preset), None, preset.name.to_string()))
    }

    fn prepare_document(&self, document: ProgramDocument) -> Option<PreparedProgram> {
        if document.schema_version != PROGRAM_SCHEMA_VERSION {
            return None;
        }
        program::prepared(document, &self.custom)
    }

    fn load_tone(&mut self, tone: Tone) -> bool {
        self.synth.load_tone(&tone)
    }

    fn install_cassette(&mut self, bay: usize) -> bool {
        if self.incoming.starts_with(b"!j106\\") {
            return self.install_librarian_cassette(bay);
        }
        self.install_sysex_cassette(bay)
    }

    fn install_librarian_cassette(&mut self, bay: usize) -> bool {
        let Some(library) =
            j106_library::parse(&self.incoming, programs::MAX_PROGRAMS_PER_CASSETTE)
        else {
            return false;
        };
        if library.programs.is_empty() {
            return false;
        }
        self.cassette_names[bay] = (!library.name.trim().is_empty()).then_some(library.name);
        self.cassettes[bay] = library
            .programs
            .into_iter()
            .enumerate()
            .map(|(index, program)| Program {
                id: format!("cassette-{}-{index:03}", bay + 1),
                name: program.name,
                tone: program.tone,
            })
            .collect();
        true
    }

    fn install_sysex_cassette(&mut self, bay: usize) -> bool {
        let mut programs = Vec::new();
        let mut offset = 0;
        while offset < self.incoming.len() {
            let Some(relative) = self.incoming[offset..]
                .iter()
                .position(|byte| *byte == 0xf0)
            else {
                break;
            };
            offset += relative;
            let Some(message) = self.incoming.get(offset..offset + APR_BYTES) else {
                break;
            };
            if let Some(apr) = Tone::decode_apr(message) {
                if programs.len() >= programs::MAX_PROGRAMS_PER_CASSETTE {
                    return false;
                }
                programs.push(Program {
                    id: format!("cassette-{}-{:03}", bay + 1, programs.len()),
                    name: format!("Imported {}", hardware_patch_code(apr.patch)),
                    tone: apr.tone,
                });
                offset += APR_BYTES;
            } else {
                offset += 1;
            }
        }
        if programs.is_empty() {
            return false;
        }
        self.cassette_names[bay] = None;
        self.cassettes[bay] = programs;
        true
    }
}

fn hardware_patch_code(number: u8) -> String {
    let group = if number < 64 { 'A' } else { 'B' };
    let cell = number % 64;
    format!("{group}{}{}", cell / 8 + 1, cell % 8 + 1)
}

fn emit<T: Serialize>(value: &T, destination: &mut [u8]) -> Option<usize> {
    let bytes = serde_json::to_vec(value).ok()?;
    let output = destination.get_mut(..bytes.len())?;
    output.copy_from_slice(&bytes);
    Some(bytes.len())
}

impl Processor for Rf106 {
    fn prepare(
        &mut self,
        sample_rate: f64,
        _maximum_frames: u32,
        input_channels: u32,
        output_channels: u32,
    ) -> bool {
        input_channels == 0 && output_channels == 2 && self.synth.prepare(sample_rate)
    }

    fn set_parameter(&mut self, index: u32, value: f64) -> bool {
        if index == PUBLIC_LFO_TRIGGER_INDEX {
            if !public_parameter_value_is_valid(index, value) {
                return false;
            }
            self.synth.set_lfo_trigger(value == 1.0);
            return true;
        }
        if index == PUBLIC_BENDER_POSITION_INDEX {
            if !public_parameter_value_is_valid(index, value) {
                return false;
            }
            self.synth.set_pitch_bend(bender_midi_value(value));
            return true;
        }
        if index == PUBLIC_CHORUS_MODE_INDEX {
            if !public_parameter_value_is_valid(index, value) {
                return false;
            }
            let (chorus_i, chorus_ii) = match value as u8 {
                0 => (0.0, 0.0),
                1 => (1.0, 0.0),
                2 => (0.0, 1.0),
                _ => return false,
            };
            return self.synth.set_parameter(27, chorus_i)
                && self.synth.set_parameter(28, chorus_ii);
        }
        if index == PUBLIC_MIDI_CHANNEL_INDEX {
            if !public_parameter_value_is_valid(index, value) {
                return false;
            }
            self.synth.reset_for_midi_channel_selection();
            return self.midi.set_basic_channel(value as u8 - 1);
        }
        if index == PUBLIC_MIDI_FUNCTION_INDEX {
            if !public_parameter_value_is_valid(index, value) {
                return false;
            }
            self.midi.set_function(match value as u8 {
                1 => MidiFunction::One,
                2 => MidiFunction::Two,
                _ => MidiFunction::Three,
            });
            return true;
        }
        if index == PUBLIC_PORTAMENTO_SWITCH_INDEX {
            if !public_parameter_value_is_valid(index, value) {
                return false;
            }
            self.synth.set_portamento_switch(value == 1.0);
            return true;
        }
        if index == PUBLIC_PHYSICAL_VOLUME_INDEX {
            if !public_parameter_value_is_valid(index, value) {
                return false;
            }
            return self.synth.set_physical_volume(value);
        }
        if index == PUBLIC_KEY_TRANSPOSE_TRIGGER_INDEX {
            if !public_parameter_value_is_valid(index, value) {
                return false;
            }
            self.key_transpose_armed = value == 1.0;
            return true;
        }
        let Some(native) = public_to_native_parameter(index) else {
            return false;
        };
        public_parameter_value_is_valid(index, value) && self.synth.set_parameter(native, value)
    }

    fn get_parameter(&self, index: u32) -> Option<f64> {
        if index == PUBLIC_LFO_TRIGGER_INDEX {
            return Some(if self.synth.lfo_trigger_active() {
                1.0
            } else {
                0.0
            });
        }
        if index == PUBLIC_BENDER_POSITION_INDEX {
            return Some(f64::from(self.synth.pitch_bend_position()));
        }
        if index == PUBLIC_CHORUS_MODE_INDEX {
            let chorus_i = self.synth.parameter(27)? >= 0.5;
            let chorus_ii = self.synth.parameter(28)? >= 0.5;
            return Some(if chorus_ii && !chorus_i {
                2.0
            } else if chorus_i {
                1.0
            } else {
                0.0
            });
        }
        if index == PUBLIC_MIDI_CHANNEL_INDEX {
            return Some(f64::from(self.midi.basic_channel() + 1));
        }
        if index == PUBLIC_MIDI_FUNCTION_INDEX {
            return Some(f64::from(self.midi.function() as u8));
        }
        if index == PUBLIC_PORTAMENTO_SWITCH_INDEX {
            return Some(if self.synth.portamento_switch_on() {
                1.0
            } else {
                0.0
            });
        }
        if index == PUBLIC_PHYSICAL_VOLUME_INDEX {
            return Some(self.synth.physical_volume());
        }
        if index == PUBLIC_KEY_TRANSPOSE_TRIGGER_INDEX {
            return Some(if self.synth.key_transpose_indicator() {
                1.0
            } else {
                0.0
            });
        }
        self.synth.parameter(public_to_native_parameter(index)?)
    }

    fn reset(&mut self) {
        self.synth.reset();
        self.midi.power_on();
        self.key_transpose_armed = false;
    }

    fn begin_resource(&mut self, id: &str, total_bytes: u64) -> bool {
        let Some(bay) = cassette_bay(id).filter(|_| total_bytes <= MAX_RESOURCE_BYTES as u64)
        else {
            return false;
        };
        self.incoming.clear();
        self.incoming.reserve(total_bytes as usize);
        self.receiving_cassette = Some(bay);
        true
    }

    fn write_resource(&mut self, offset: u64, bytes: &[u8]) -> bool {
        if self.receiving_cassette.is_none()
            || offset != self.incoming.len() as u64
            || self.incoming.len() + bytes.len() > MAX_RESOURCE_BYTES
        {
            self.receiving_cassette = None;
            return false;
        }
        self.incoming.extend_from_slice(bytes);
        true
    }

    fn end_resource(&mut self) -> bool {
        let Some(bay) = self.receiving_cassette.take() else {
            return false;
        };
        let accepted = self.install_cassette(bay);
        self.incoming.clear();
        accepted
    }

    fn write_program_catalog(&mut self, destination: &mut [u8]) -> Option<usize> {
        catalog::write(
            &self.cassettes,
            &self.cassette_names,
            &self.custom,
            destination,
        )
    }

    fn load_preset(&mut self, id: &str) -> bool {
        if let Some(document_id) = CustomPrograms::document_id(id) {
            let Some(tone) = self.custom.find(document_id).map(|program| program.tone) else {
                return false;
            };
            return self.load_tone(tone);
        }
        if let Some((bay, index)) = cassette_program(id) {
            let Some(tone) = self
                .cassettes
                .get(bay)
                .and_then(|cassette| cassette.get(index))
                .map(|program| program.tone)
            else {
                return false;
            };
            return self.load_tone(tone);
        }
        match parse_preset_id(id) {
            Some(index) if self.synth.load_factory_preset(index) => {
                self.selected_factory_program = index;
                true
            }
            _ => false,
        }
    }

    fn save_state(&self, destination: &mut [u8]) -> Option<usize> {
        let destination = destination.get_mut(..STATE_BYTES)?;
        destination[..4].copy_from_slice(&STATE_MAGIC);
        destination[4..8].copy_from_slice(&STATE_SCHEMA_VERSION.to_le_bytes());
        destination[8..12].copy_from_slice(&self.selected_factory_program.to_le_bytes());
        for (index, value) in self.synth.parameters().iter().enumerate() {
            let offset = 12 + index * 8;
            destination[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
        }
        destination[MIDI_STATE_OFFSET] = self.midi.basic_channel();
        destination[MIDI_STATE_OFFSET + 1] = self.midi.function() as u8;
        destination[PORTAMENTO_STATE_OFFSET] = u8::from(self.synth.portamento_switch_on());
        destination[VOLUME_STATE_OFFSET..STATE_BYTES]
            .copy_from_slice(&self.synth.physical_volume().to_le_bytes());
        Some(STATE_BYTES)
    }

    fn load_state(&mut self, state: &[u8]) -> bool {
        if state.len() != STATE_BYTES || state[..4] != STATE_MAGIC {
            return false;
        }
        let version = u32::from_le_bytes(state[4..8].try_into().unwrap_or([0; 4]));
        if version != STATE_SCHEMA_VERSION {
            return false;
        }
        let program = u32::from_le_bytes(state[8..12].try_into().unwrap_or([0; 4]));
        if factory_preset(program).is_none() {
            return false;
        }
        let mut parameters = [0.0; NATIVE_PARAMETER_COUNT];
        for (index, value) in parameters.iter_mut().enumerate() {
            let offset = 12 + index * 8;
            let Ok(bytes) = <[u8; 8]>::try_from(&state[offset..offset + 8]) else {
                return false;
            };
            *value = f64::from_le_bytes(bytes);
        }
        let basic_channel = state[MIDI_STATE_OFFSET];
        if basic_channel > 15 {
            return false;
        }
        let function = match state[MIDI_STATE_OFFSET + 1] {
            1 => MidiFunction::One,
            2 => MidiFunction::Two,
            3 => MidiFunction::Three,
            _ => return false,
        };
        let portamento_switch_on = match state[PORTAMENTO_STATE_OFFSET] {
            0 => false,
            1 => true,
            _ => return false,
        };
        let Ok(volume_bytes) = <[u8; 8]>::try_from(&state[VOLUME_STATE_OFFSET..STATE_BYTES]) else {
            return false;
        };
        let physical_volume = f64::from_le_bytes(volume_bytes);
        if !public_parameter_value_is_valid(PUBLIC_PHYSICAL_VOLUME_INDEX, physical_volume) {
            return false;
        }
        if !self.synth.load_parameters(&parameters) {
            return false;
        }
        self.synth.set_portamento_switch(portamento_switch_on);
        if !self.synth.set_physical_volume(physical_volume) {
            return false;
        }
        let mut midi = MidiReceiver::new();
        midi.set_function(function);
        if !midi.set_basic_channel(basic_channel) {
            return false;
        }
        // RF-106 starts in Omni On while retaining the selected basic channel
        // and Function switch position.
        midi.power_on();
        self.midi = midi;
        self.selected_factory_program = program;
        self.key_transpose_armed = false;
        true
    }

    fn program_editing_capabilities(&self) -> u32 {
        PROGRAM_EDIT_BASIC | PROGRAM_EDIT_PREVIEW | PROGRAM_EDIT_DECLARATIVE
    }

    fn begin_program_edit(&mut self, request: &[u8], destination: &mut [u8]) -> Option<usize> {
        let request: ProgramEditRequest = serde_json::from_slice(request).ok()?;
        if request.schema_version != PROGRAM_EDIT_SCHEMA_VERSION {
            return None;
        }
        let (tone, id, name) = match request.program_id.as_deref() {
            Some(catalog_id) => {
                let (tone, existing, name) = self.tone_for_catalog_id(catalog_id)?;
                (
                    tone,
                    existing.unwrap_or_else(|| self.custom.next_id()),
                    name,
                )
            }
            None => (
                Tone::from_native(self.synth.parameters())?,
                self.custom.next_id(),
                "RF NEW".to_string(),
            ),
        };
        let document = program::document(&id, &name, tone);
        emit(&self.prepare_document(document)?, destination)
    }

    fn prepare_program_save(&mut self, document: &[u8], destination: &mut [u8]) -> Option<usize> {
        let document: ProgramDocument = serde_json::from_slice(document).ok()?;
        emit(&self.prepare_document(document)?, destination)
    }

    fn program_editor_view(&mut self, document: &[u8], destination: &mut [u8]) -> Option<usize> {
        let document: ProgramDocument = serde_json::from_slice(document).ok()?;
        emit(&program::editor_view(&document)?, destination)
    }

    fn apply_program_edit(&mut self, request: &[u8], destination: &mut [u8]) -> Option<usize> {
        let request: ProgramFieldEditRequest = serde_json::from_slice(request).ok()?;
        let document = program::apply_edit(request)?;
        emit(&self.prepare_document(document)?, destination)
    }

    fn install_program(&mut self, prepared: &[u8]) -> bool {
        let prepared: PreparedProgram = match serde_json::from_slice(prepared) {
            Ok(prepared) => prepared,
            Err(_) => return false,
        };
        let Some(prepared) = self.prepare_document(prepared.document) else {
            return false;
        };
        let Some(program) = program::program_of(&prepared.document) else {
            return false;
        };
        self.custom.install(program)
    }

    fn preview_program(&mut self, prepared: &[u8]) -> bool {
        let prepared: PreparedProgram = match serde_json::from_slice(prepared) {
            Ok(prepared) => prepared,
            Err(_) => return false,
        };
        match program::tone_of(&prepared.document) {
            Some(tone) => self.load_tone(tone),
            None => false,
        }
    }

    fn process(
        &mut self,
        _input: &[f32],
        output: &mut [f32],
        midi: &[MidiEvent],
        parameters: &[ParameterEvent],
        frames: u32,
        input_channels: u32,
        output_channels: u32,
    ) {
        if input_channels != 0 || output_channels != 2 {
            output.fill(0.0);
            return;
        }
        let mut midi_index = 0;
        let mut parameter_index = 0;
        for frame in 0..frames {
            while let Some(event) = parameters.get(parameter_index) {
                if event.frame != frame {
                    break;
                }
                let _ = self.set_parameter(event.index, event.value);
                parameter_index += 1;
            }
            while let Some(event) = midi.get(midi_index) {
                if event.frame != frame {
                    break;
                }
                self.apply_midi(*event);
                midi_index += 1;
            }
            let (left, right) = self.synth.process_circuit_sample();
            let master = self.synth.parameter(NATIVE_HOST_MASTER).unwrap_or(0.0) as f32;
            let offset = frame as usize * 2;
            output[offset] = protect_host_output(left, master);
            output[offset + 1] = protect_host_output(right, master);
        }
    }
}

impl rackforge_plugin_sdk::ParallelProcessor for Rf106 {
    type Unit = ParallelVoiceUnit;

    fn prepare(
        &mut self,
        sample_rate: f64,
        maximum_frames: u32,
        input_channels: u32,
        output_channels: u32,
    ) -> bool {
        if input_channels != 0
            || output_channels != MAX_OUTPUT_CHANNELS as u32
            || maximum_frames as usize > MAX_FRAMES
        {
            return false;
        }
        self.synth.capture_voice_commands(true);
        self.synth.prepare(sample_rate)
    }

    fn set_parameter(&mut self, index: u32, value: f64) -> bool {
        Processor::set_parameter(self, index, value)
    }

    fn get_parameter(&self, index: u32) -> Option<f64> {
        Processor::get_parameter(self, index)
    }

    fn reset(&mut self) {
        Processor::reset(self)
    }

    fn begin_resource(&mut self, id: &str, total_bytes: u64) -> bool {
        Processor::begin_resource(self, id, total_bytes)
    }

    fn write_resource(&mut self, offset: u64, bytes: &[u8]) -> bool {
        Processor::write_resource(self, offset, bytes)
    }

    fn end_resource(&mut self) -> bool {
        Processor::end_resource(self)
    }

    fn write_program_catalog(&mut self, destination: &mut [u8]) -> Option<usize> {
        Processor::write_program_catalog(self, destination)
    }

    fn load_preset(&mut self, id: &str) -> bool {
        Processor::load_preset(self, id)
    }

    fn save_state(&self, destination: &mut [u8]) -> Option<usize> {
        Processor::save_state(self, destination)
    }

    fn load_state(&mut self, state: &[u8]) -> bool {
        Processor::load_state(self, state)
    }

    fn program_editing_capabilities(&self) -> u32 {
        Processor::program_editing_capabilities(self)
    }

    fn begin_program_edit(&mut self, request: &[u8], destination: &mut [u8]) -> Option<usize> {
        Processor::begin_program_edit(self, request, destination)
    }

    fn prepare_program_save(&mut self, document: &[u8], destination: &mut [u8]) -> Option<usize> {
        Processor::prepare_program_save(self, document, destination)
    }

    fn install_program(&mut self, prepared: &[u8]) -> bool {
        Processor::install_program(self, prepared)
    }

    fn preview_program(&mut self, prepared: &[u8]) -> bool {
        Processor::preview_program(self, prepared)
    }

    fn program_editor_view(&mut self, document: &[u8], destination: &mut [u8]) -> Option<usize> {
        Processor::program_editor_view(self, document, destination)
    }

    fn apply_program_edit(&mut self, request: &[u8], destination: &mut [u8]) -> Option<usize> {
        Processor::apply_program_edit(self, request, destination)
    }

    fn begin_block(&mut self, context: &BlockContext<'_>, plan: &mut PlanWriter<'_>) {
        let frames = context.frames as usize;
        if frames == 0 || frames > MAX_FRAMES {
            return;
        }
        for commands in &mut self.commands {
            commands.clear();
        }
        self.command_overflow = false;
        let initial_epoch = self.synth.voice_epoch();
        self.capture_voice_commands(0);

        let header = SharedHeader {
            magic: SHARED_MAGIC,
            version: PARALLEL_WIRE_VERSION,
            frames: context.frames,
            sample_rate_bits: self.synth.sample_rate().to_bits(),
        };
        let mut shared_offset = 0;
        write_value(plan.shared_buffer(), &mut shared_offset, &header);

        let mut midi_index = 0;
        let mut parameter_index = 0;
        for frame in 0..frames {
            while let Some(event) = context.parameters.get(parameter_index) {
                if event.frame as usize != frame {
                    break;
                }
                let _ = Processor::set_parameter(self, event.index, event.value);
                self.capture_voice_commands(frame as u32);
                parameter_index += 1;
            }
            while let Some(event) = context.midi.get(midi_index) {
                if event.frame as usize != frame {
                    break;
                }
                self.apply_midi(*event);
                self.capture_voice_commands(frame as u32);
                midi_index += 1;
            }

            let prepared = self.synth.prepare_next_sample();
            self.end_frames[frame] = EndFrame {
                finish: prepared.finish,
                host_master: self.synth.parameter(NATIVE_HOST_MASTER).unwrap_or(0.0) as f32,
            };
            write_value(plan.shared_buffer(), &mut shared_offset, &prepared.common);
        }

        if !plan.commit_shared(shared_offset) || self.command_overflow {
            return;
        }
        for unit in 0..VOICE_COUNT {
            let payload_bytes = self.write_dispatch(unit, frames, initial_epoch);
            let activated = plan.activate(unit as u32, &self.dispatch_scratch[..payload_bytes]);
            debug_assert!(activated);
        }
    }

    fn render_unit(
        unit_index: u32,
        unit: &mut Self::Unit,
        payload: &[u8],
        context: &UnitContext<'_>,
        output: &mut [f32],
    ) {
        let channels = context.output_channels as usize;
        let samples = context.frames as usize * channels;
        output[..samples].fill(0.0);
        if channels != MAX_OUTPUT_CHANNELS {
            return;
        }
        let Some((shared_header, mut shared_offset)) =
            read_value::<SharedHeader>(context.shared, 0)
        else {
            return;
        };
        let Some((dispatch_header, command_offset)) = read_value::<DispatchHeader>(payload, 0)
        else {
            return;
        };
        if shared_header.magic != SHARED_MAGIC
            || shared_header.version != PARALLEL_WIRE_VERSION
            || shared_header.frames != context.frames
            || dispatch_header.magic != DISPATCH_MAGIC
            || dispatch_header.version != PARALLEL_WIRE_VERSION
            || dispatch_header.frames != context.frames
            || dispatch_header.command_count as usize > MAX_COMMANDS_PER_UNIT
        {
            return;
        }
        let sample_rate = f32::from_bits(shared_header.sample_rate_bits);
        if !sample_rate.is_finite() || sample_rate <= 0.0 {
            return;
        }
        let unit_index = unit_index as usize;
        unit.synchronize_epoch(dispatch_header.initial_epoch, unit_index);
        let mut command_index = 0;
        for frame in 0..context.frames as usize {
            while command_index < dispatch_header.command_count as usize {
                let offset = command_offset + command_index * mem::size_of::<WireVoiceCommand>();
                let Some((wire, _)) = read_value::<WireVoiceCommand>(payload, offset) else {
                    return;
                };
                if wire.frame as usize != frame {
                    break;
                }
                let Some(command) = wire.decode() else {
                    return;
                };
                unit.apply_command(command, unit_index);
                command_index += 1;
            }
            let Some((common, next_shared)) =
                read_value::<CommonVoiceFrame>(context.shared, shared_offset)
            else {
                return;
            };
            shared_offset = next_shared;
            let sample = unit.next(sample_rate, common);
            let offset = frame * channels;
            output[offset] = sample;
            output[offset + 1] = sample;
        }
    }

    fn end_block(
        &mut self,
        mix: &UnitMix<'_>,
        output: &mut [f32],
        frames: u32,
        output_channels: u32,
    ) {
        let channels = output_channels as usize;
        if channels != MAX_OUTPUT_CHANNELS {
            output.fill(0.0);
            return;
        }
        for frame in 0..frames as usize {
            let mut voice_sum = 0.0;
            for unit in mix.active_units() {
                voice_sum += mix.slot(unit)[frame * channels];
            }
            let end = self.end_frames[frame];
            let (left, right) = self.synth.finish_prepared_sample(end.finish, voice_sum);
            let offset = frame * channels;
            output[offset] = protect_host_output(left, end.host_master);
            output[offset + 1] = protect_host_output(right, end.host_master);
        }
    }
}

fn write_value<T: Copy>(destination: &mut [u8], offset: &mut usize, value: &T) {
    let bytes = unsafe {
        // SAFETY: the private coordinator/unit wire format contains only
        // initialized Copy values from this exact component build.
        slice::from_raw_parts((value as *const T).cast::<u8>(), mem::size_of::<T>())
    };
    destination[*offset..*offset + bytes.len()].copy_from_slice(bytes);
    *offset += bytes.len();
}

fn write_values<T: Copy>(destination: &mut [u8], offset: &mut usize, values: &[T]) {
    let byte_count = mem::size_of_val(values);
    let bytes = unsafe {
        // SAFETY: same private wire format as `write_value`; the source slice
        // remains live for the duration of the copy.
        slice::from_raw_parts(values.as_ptr().cast::<u8>(), byte_count)
    };
    destination[*offset..*offset + byte_count].copy_from_slice(bytes);
    *offset += byte_count;
}

fn read_value<T: Copy>(source: &[u8], offset: usize) -> Option<(T, usize)> {
    let end = offset.checked_add(mem::size_of::<T>())?;
    let bytes = source.get(offset..end)?;
    let value = unsafe {
        // SAFETY: coordinator and unit decode the same private payload and use
        // an explicit unaligned read from a bounds-checked byte range.
        core::ptr::read_unaligned(bytes.as_ptr().cast::<T>())
    };
    Some((value, end))
}

/// RackForge-only gain and fault containment. This is intentionally outside
/// `rf-106-dsp`: it is not part of the RF-106 circuit model.
fn protect_host_output(sample: f32, master: f32) -> f32 {
    if !sample.is_finite() || !master.is_finite() {
        return 0.0;
    }
    let calibrated = sample * master.clamp(0.0, 1.0) * HOST_GAIN_AT_UNITY;
    let magnitude = calibrated.abs();
    if magnitude <= HOST_LIMIT_THRESHOLD {
        return calibrated;
    }
    let knee = HOST_LIMIT_CEILING - HOST_LIMIT_THRESHOLD;
    let limited =
        HOST_LIMIT_THRESHOLD + knee * libm::tanhf((magnitude - HOST_LIMIT_THRESHOLD) / knee);
    calibrated.signum() * limited
}

fn bender_midi_value(position: f64) -> u16 {
    let position = position.clamp(-1.0, 1.0);
    if position < 0.0 {
        libm::round(8_192.0 * (position + 1.0)) as u16
    } else {
        libm::round(8_192.0 + 8_191.0 * position) as u16
    }
}

impl Rf106 {
    fn apply_midi(&mut self, event: MidiEvent) {
        let Some(action) =
            self.midi
                .receive(event.data[0], event.data[1], event.data[2], event.length)
        else {
            return;
        };
        match action {
            MidiReceiveAction::NoteOn { note, velocity: _ } if self.key_transpose_armed => {
                if let Some(key_index) = note.checked_sub(36).filter(|key| *key <= 60) {
                    let _ = self.synth.select_key_transpose_key(key_index);
                }
            }
            MidiReceiveAction::NoteOff { .. } if self.key_transpose_armed => {}
            MidiReceiveAction::NoteOn { note, velocity } => self.synth.note_on(note, velocity),
            MidiReceiveAction::NoteOff { note } => self.synth.note_off(note),
            MidiReceiveAction::Hold { enabled } => self.synth.set_hold(enabled),
            MidiReceiveAction::Modulation { value } => self.synth.control_change(1, value),
            MidiReceiveAction::PitchBend { value } => self.synth.set_pitch_bend(value),
            MidiReceiveAction::AllNotesOff => self.synth.all_notes_off(),
        }
    }
}

export_parallel_processor!(
    Rf106,
    max_units = VOICE_COUNT,
    dispatch_stride = DISPATCH_STRIDE,
    shared_capacity = SHARED_CAPACITY,
    max_frames = 4096,
    max_input_channels = 0,
    max_output_channels = 2,
    max_midi_events = 1024,
    max_parameter_events = 1024,
    max_transfer_bytes = 262144
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard};

    static PARALLEL_EXPORT_TEST_LOCK: Mutex<()> = Mutex::new(());

    fn parallel_export_test_guard() -> MutexGuard<'static, ()> {
        PARALLEL_EXPORT_TEST_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    #[test]
    fn processor_renders_and_round_trips_opaque_state() {
        let mut plugin = Rf106::default();
        assert!(plugin.prepare(48_000.0, 256, 0, 2));
        assert!(plugin.load_preset("factory.rf106.000"));
        let mut state = [0; 1024];
        let length = plugin.save_state(&mut state).unwrap();
        assert_eq!(length, STATE_BYTES);
        assert!(plugin.set_parameter(1, 0.2));
        assert!(plugin.load_state(&state[..length]));

        let mut output = [0.0; 512];
        plugin.process(
            &[],
            &mut output,
            &[MidiEvent::new(0, [0x90, 60, 110], 3).unwrap()],
            &[],
            256,
            0,
            2,
        );
        assert!(output.iter().all(|sample| sample.is_finite()));
        assert!(output.iter().any(|sample| sample.abs() > 0.0001));
    }

    #[test]
    fn parallel_voice_contract_is_bit_exact_with_the_sequential_engine() {
        let _guard = parallel_export_test_guard();
        const FRAMES: u32 = 256;
        let mut reference = Rf106::default();
        reference.synth.capture_voice_commands(true);
        assert!(Processor::prepare(&mut reference, 48_000.0, FRAMES, 0, 2));
        let mut parallel = RackForgeParallelExport::default();
        assert!(Processor::prepare(&mut parallel, 48_000.0, FRAMES, 0, 2));
        assert!(Processor::load_preset(&mut reference, "factory.rf106.000"));
        assert!(Processor::load_preset(&mut parallel, "factory.rf106.000"));

        let scripts: [(&[MidiEvent], &[ParameterEvent]); 5] = [
            (
                &[
                    MidiEvent {
                        frame: 3,
                        data: [0x90, 48, 110],
                        length: 3,
                    },
                    MidiEvent {
                        frame: 11,
                        data: [0x90, 55, 100],
                        length: 3,
                    },
                    MidiEvent {
                        frame: 29,
                        data: [0x90, 60, 96],
                        length: 3,
                    },
                ],
                &[],
            ),
            (
                &[MidiEvent {
                    frame: 41,
                    data: [0xb0, 64, 127],
                    length: 3,
                }],
                &[ParameterEvent {
                    frame: 17,
                    index: PUBLIC_CHORUS_MODE_INDEX,
                    value: 2.0,
                }],
            ),
            (
                &[
                    MidiEvent {
                        frame: 23,
                        data: [0x80, 48, 0],
                        length: 3,
                    },
                    MidiEvent {
                        frame: 47,
                        data: [0x80, 55, 0],
                        length: 3,
                    },
                ],
                &[],
            ),
            (
                &[
                    MidiEvent {
                        frame: 7,
                        data: [0xb0, 64, 0],
                        length: 3,
                    },
                    MidiEvent {
                        frame: 101,
                        data: [0xe0, 0x7f, 0x7f],
                        length: 3,
                    },
                ],
                &[ParameterEvent {
                    frame: 67,
                    index: PUBLIC_PHYSICAL_VOLUME_INDEX,
                    value: 0.73,
                }],
            ),
            (
                &[MidiEvent {
                    frame: 31,
                    data: [0xb0, 123, 0],
                    length: 3,
                }],
                &[],
            ),
        ];

        for (block, (midi, parameters)) in scripts.into_iter().enumerate() {
            let mut expected = [0.0_f32; FRAMES as usize * 2];
            let mut actual = [0.0_f32; FRAMES as usize * 2];
            Processor::process(
                &mut reference,
                &[],
                &mut expected,
                midi,
                parameters,
                FRAMES,
                0,
                2,
            );
            Processor::process(
                &mut parallel,
                &[],
                &mut actual,
                midi,
                parameters,
                FRAMES,
                0,
                2,
            );
            assert_eq!(
                actual, expected,
                "parallel render diverged at block {block}"
            );
        }
    }

    #[test]
    fn original_factory_program_is_audible_after_preset_and_state_loads() {
        fn render_peak(plugin: &mut Rf106) -> f32 {
            let mut output = [0.0; 8_192];
            plugin.process(
                &[],
                &mut output,
                &[MidiEvent::new(0, [0x90, 60, 127], 3).unwrap()],
                &[],
                4_096,
                0,
                2,
            );
            output
                .iter()
                .fold(0.0_f32, |peak, sample| peak.max(sample.abs()))
        }

        let mut selected = Rf106::default();
        assert!(selected.prepare(48_000.0, 4_096, 0, 2));
        assert!(selected.load_preset("factory.rf106.000"));
        let mut state = [0; 1_024];
        let state_length = selected.save_state(&mut state).unwrap();
        let selected_peak = render_peak(&mut selected);
        assert!(selected_peak > 0.02, "selected_peak={selected_peak}");

        let mut restored = Rf106::default();
        assert!(restored.prepare(48_000.0, 4_096, 0, 2));
        assert!(restored.load_state(&state[..state_length]));
        let restored_peak = render_peak(&mut restored);
        assert!(restored_peak > 0.02, "restored_peak={restored_peak}");
    }

    #[test]
    fn host_output_is_linear_below_its_safety_knee() {
        for master in [0.0_f32, 0.25, 0.5, 1.0] {
            for sample in [-1.0_f32, -0.5, 0.0, 0.5, 1.0] {
                assert_eq!(
                    protect_host_output(sample, master),
                    sample * master * HOST_GAIN_AT_UNITY
                );
            }
        }
    }

    #[test]
    fn host_output_contains_only_exceptional_or_invalid_values() {
        assert_eq!(protect_host_output(f32::NAN, 1.0), 0.0);
        assert_eq!(protect_host_output(f32::INFINITY, 1.0), 0.0);
        assert_eq!(protect_host_output(1.0, f32::NAN), 0.0);
        assert_eq!(protect_host_output(1.0, -1.0), 0.0);
        assert_eq!(protect_host_output(1.0, 2.0), HOST_GAIN_AT_UNITY);

        let positive = protect_host_output(100.0, 1.0);
        let negative = protect_host_output(-100.0, 1.0);
        assert!((HOST_LIMIT_THRESHOLD..=HOST_LIMIT_CEILING).contains(&positive));
        assert!((-HOST_LIMIT_CEILING..=-HOST_LIMIT_THRESHOLD).contains(&negative));
    }

    #[test]
    fn midi_adapter_obeys_omni_and_channel_mode_messages() {
        let mut plugin = Rf106::default();
        plugin.apply_midi(MidiEvent::new(0, [0x91, 60, 100], 3).unwrap());
        assert_ne!(plugin.synth.allocator_gate_mask(), 0);

        // Unlike a modern synth, CC123 is inert on A_5.
        plugin.apply_midi(MidiEvent::new(0, [0xb0, 123, 0], 3).unwrap());
        assert_ne!(plugin.synth.allocator_gate_mask(), 0);

        // CC124 on the basic channel releases and enters Omni Off.
        plugin.apply_midi(MidiEvent::new(0, [0xb0, 124, 0], 3).unwrap());
        assert_eq!(plugin.synth.allocator_gate_mask(), 0);
        plugin.apply_midi(MidiEvent::new(0, [0x91, 62, 100], 3).unwrap());
        assert_eq!(plugin.synth.allocator_gate_mask(), 0);
        plugin.apply_midi(MidiEvent::new(0, [0x90, 62, 100], 3).unwrap());
        assert_ne!(plugin.synth.allocator_gate_mask(), 0);
    }

    #[test]
    fn public_lfo_trigger_is_momentary_and_outside_persisted_patch_state() {
        let mut plugin = Rf106::default();
        assert_eq!(plugin.get_parameter(PUBLIC_LFO_TRIGGER_INDEX), Some(0.0));
        assert!(plugin.set_parameter(PUBLIC_LFO_TRIGGER_INDEX, 1.0));
        assert_eq!(plugin.get_parameter(PUBLIC_LFO_TRIGGER_INDEX), Some(1.0));
        assert!(!plugin.set_parameter(PUBLIC_LFO_TRIGGER_INDEX, 0.5));

        assert!(plugin.load_preset("factory.rf106.001"));
        assert_eq!(plugin.get_parameter(PUBLIC_LFO_TRIGGER_INDEX), Some(1.0));
        let mut state = [0; 1024];
        let length = plugin.save_state(&mut state).unwrap();
        assert_eq!(length, STATE_BYTES);

        assert!(plugin.set_parameter(PUBLIC_LFO_TRIGGER_INDEX, 0.0));
        assert!(plugin.load_state(&state[..length]));
        assert_eq!(plugin.get_parameter(PUBLIC_LFO_TRIGGER_INDEX), Some(0.0));
    }

    #[test]
    fn panel_bender_uses_the_same_spring_centred_path_as_midi_pitch_bend() {
        let mut plugin = Rf106::default();
        for (position, expected_midi) in [(-1.0, 0), (0.0, 8_192), (1.0, 16_383)] {
            assert!(plugin.set_parameter(PUBLIC_BENDER_POSITION_INDEX, position));
            assert_eq!(bender_midi_value(position), expected_midi);
            assert!(
                (plugin.get_parameter(PUBLIC_BENDER_POSITION_INDEX).unwrap() - position).abs()
                    < 1.0e-6
            );
        }
        assert!(!plugin.set_parameter(PUBLIC_BENDER_POSITION_INDEX, 1.01));
        assert!(!plugin.set_parameter(PUBLIC_BENDER_POSITION_INDEX, f64::NAN));
    }

    #[test]
    fn complete_audio_panel_maps_to_the_production_engine() {
        let mut plugin = Rf106::default();
        for (public, native, value) in [
            (14, 3, 0.25),
            (15, 4, 0.50),
            (16, 29, 2.0),
            (17, 5, 0.75),
            (18, 6, 1.0),
            (19, 33, 1.0),
            (20, 23, 1.0),
            (21, 24, 1.0),
            (22, 7, 0.625),
            (23, 8, 0.375),
            (24, 9, 3.0),
            (25, 12, 0.5),
            (26, 34, 1.0),
            (27, 13, 0.75),
            (28, 14, 1.0),
            (29, 35, 1.0),
            (30, 15, 0.5),
            (32, 38, 1.0),
        ] {
            assert!(plugin.set_parameter(public, value));
            assert_eq!(plugin.synth.parameter(native), Some(value));
            assert_eq!(plugin.get_parameter(public), Some(value));
        }
    }

    #[test]
    fn chorus_selector_exposes_the_three_firmware_states() {
        let mut plugin = Rf106::default();
        for (mode, chorus_i, chorus_ii) in [(0.0, 0.0, 0.0), (1.0, 1.0, 0.0), (2.0, 0.0, 1.0)] {
            assert!(plugin.set_parameter(PUBLIC_CHORUS_MODE_INDEX, mode));
            assert_eq!(plugin.synth.parameter(27), Some(chorus_i));
            assert_eq!(plugin.synth.parameter(28), Some(chorus_ii));
            assert_eq!(plugin.get_parameter(PUBLIC_CHORUS_MODE_INDEX), Some(mode));
        }
        assert!(!plugin.set_parameter(PUBLIC_CHORUS_MODE_INDEX, 3.0));
        assert!(!plugin.set_parameter(PUBLIC_CHORUS_MODE_INDEX, 1.5));
    }

    #[test]
    fn midi_channel_selection_routes_input_and_performs_the_musical_reset() {
        let mut plugin = Rf106::default();
        plugin.apply_midi(MidiEvent::new(0, [0x90, 60, 100], 3).unwrap());
        assert_ne!(plugin.synth.allocator_gate_mask(), 0);

        assert!(plugin.set_parameter(PUBLIC_MIDI_CHANNEL_INDEX, 10.0));
        assert_eq!(plugin.get_parameter(PUBLIC_MIDI_CHANNEL_INDEX), Some(10.0));
        assert_eq!(plugin.synth.allocator_gate_mask(), 0);
        assert!(!plugin.midi.is_omni());

        plugin.apply_midi(MidiEvent::new(0, [0x90, 62, 100], 3).unwrap());
        assert_eq!(plugin.synth.allocator_gate_mask(), 0);
        plugin.apply_midi(MidiEvent::new(0, [0x99, 62, 100], 3).unwrap());
        assert_ne!(plugin.synth.allocator_gate_mask(), 0);
    }

    #[test]
    fn held_key_transpose_button_converts_the_next_keyboard_note_into_a_selection() {
        let mut plugin = Rf106::default();
        assert_eq!(
            plugin.get_parameter(PUBLIC_KEY_TRANSPOSE_TRIGGER_INDEX),
            Some(0.0)
        );
        assert!(plugin.set_parameter(PUBLIC_KEY_TRANSPOSE_TRIGGER_INDEX, 1.0));
        plugin.apply_midi(MidiEvent::new(0, [0x90, 67, 100], 3).unwrap());
        assert_eq!(plugin.synth.key_transpose_offset(), 7);
        assert_eq!(plugin.synth.allocator_gate_mask(), 0);
        assert!(plugin.set_parameter(PUBLIC_KEY_TRANSPOSE_TRIGGER_INDEX, 0.0));
        assert_eq!(
            plugin.get_parameter(PUBLIC_KEY_TRANSPOSE_TRIGGER_INDEX),
            Some(1.0)
        );

        plugin.apply_midi(MidiEvent::new(0, [0x90, 36, 100], 3).unwrap());
        assert_ne!(plugin.synth.allocator_gate_mask(), 0);
    }

    #[test]
    fn hardware_controls_round_trip_in_native_state() {
        let mut plugin = Rf106::default();
        assert!(plugin.set_parameter(PUBLIC_MIDI_CHANNEL_INDEX, 10.0));
        assert!(plugin.set_parameter(PUBLIC_MIDI_FUNCTION_INDEX, 1.0));
        assert!(plugin.set_parameter(PUBLIC_PORTAMENTO_SWITCH_INDEX, 0.0));
        assert!(plugin.set_parameter(PUBLIC_PHYSICAL_VOLUME_INDEX, 0.42));

        let mut state = [0; 1024];
        let length = plugin.save_state(&mut state).unwrap();
        assert_eq!(length, STATE_BYTES);
        assert_eq!(state[MIDI_STATE_OFFSET], 9);
        assert_eq!(state[MIDI_STATE_OFFSET + 1], MidiFunction::One as u8);
        assert_eq!(state[PORTAMENTO_STATE_OFFSET], 0);
        assert_eq!(
            f64::from_le_bytes(state[VOLUME_STATE_OFFSET..STATE_BYTES].try_into().unwrap()),
            0.42
        );

        let mut restored = Rf106::default();
        assert!(restored.load_state(&state[..length]));
        assert_eq!(
            restored.get_parameter(PUBLIC_MIDI_CHANNEL_INDEX),
            Some(10.0)
        );
        assert_eq!(
            restored.get_parameter(PUBLIC_MIDI_FUNCTION_INDEX),
            Some(1.0)
        );
        assert_eq!(
            restored.get_parameter(PUBLIC_PORTAMENTO_SWITCH_INDEX),
            Some(0.0)
        );
        assert_eq!(
            restored.get_parameter(PUBLIC_PHYSICAL_VOLUME_INDEX),
            Some(0.42)
        );
        assert!(restored.midi.is_omni());
    }

    #[test]
    fn state_rejects_invalid_midi_hardware_values() {
        let plugin = Rf106::default();
        let mut state = [0; 1024];
        let length = plugin.save_state(&mut state).unwrap();

        state[MIDI_STATE_OFFSET] = 16;
        assert!(!Rf106::default().load_state(&state[..length]));
        state[MIDI_STATE_OFFSET] = 0;
        state[MIDI_STATE_OFFSET + 1] = 4;
        assert!(!Rf106::default().load_state(&state[..length]));
        state[MIDI_STATE_OFFSET + 1] = MidiFunction::Three as u8;
        state[PORTAMENTO_STATE_OFFSET] = 2;
        assert!(!Rf106::default().load_state(&state[..length]));
        state[PORTAMENTO_STATE_OFFSET] = 1;
        state[VOLUME_STATE_OFFSET..STATE_BYTES].copy_from_slice(&f64::NAN.to_le_bytes());
        assert!(!Rf106::default().load_state(&state[..length]));
        state[VOLUME_STATE_OFFSET..STATE_BYTES].copy_from_slice(&1.01_f64.to_le_bytes());
        assert!(!Rf106::default().load_state(&state[..length]));
    }

    #[test]
    fn obsolete_state_schema_is_rejected() {
        let plugin = Rf106::default();
        let mut state = [0; 1024];
        let length = plugin.save_state(&mut state).unwrap();
        state[4..8].copy_from_slice(&1_u32.to_le_bytes());
        assert!(!Rf106::default().load_state(&state[..length]));
    }

    #[test]
    fn each_sysex_cassette_becomes_an_independent_editable_program_bank() {
        let first = Tone::from_factory(factory_preset(0).unwrap())
            .encode_apr(rf_106_contract::sysex::APR_PROGRAM, 0, 0)
            .unwrap();
        let second = Tone::from_factory(factory_preset(1).unwrap())
            .encode_apr(rf_106_contract::sysex::APR_PROGRAM, 0, 1)
            .unwrap();
        let mut bank = Vec::from(first);
        bank.extend_from_slice(&second);

        let mut plugin = Rf106::default();
        assert!(plugin.begin_resource("cassette-3", bank.len() as u64));
        assert!(plugin.write_resource(0, &bank[..17]));
        assert!(plugin.write_resource(17, &bank[17..]));
        assert!(plugin.end_resource());
        assert_eq!(plugin.cassettes[2].len(), 2);
        assert!(plugin.cassettes[0].is_empty());
        assert!(plugin.load_preset("cassette.rf106.3.001"));

        assert!(!plugin.begin_resource("program-bank", first.len() as u64));
        assert!(plugin.begin_resource("cassette-8", first.len() as u64));
        assert!(plugin.write_resource(0, &first));
        assert!(plugin.end_resource());
        assert_eq!(plugin.cassettes[2].len(), 2);
        assert_eq!(plugin.cassettes[7].len(), 1);

        let mut output = vec![0; 64 * 1024];
        let length = plugin.write_program_catalog(&mut output).unwrap();
        let catalog: serde_json::Value = serde_json::from_slice(&output[..length]).unwrap();
        assert!(
            catalog["banks"]
                .as_array()
                .unwrap()
                .iter()
                .any(|bank| bank["id"] == "cassette.rf106.3")
        );
        assert!(
            catalog["banks"]
                .as_array()
                .unwrap()
                .iter()
                .any(|bank| bank["id"] == "cassette.rf106.8")
        );
        assert!(
            catalog["presets"]
                .as_array()
                .unwrap()
                .iter()
                .any(|program| {
                    program["id"] == "cassette.rf106.3.001" && program["editable"] == true
                })
        );
        assert!(
            !catalog["banks"]
                .as_array()
                .unwrap()
                .iter()
                .any(|bank| bank["id"] == "cassette.rf106.1")
        );
    }

    #[test]
    fn program_editing_installs_a_saved_program_and_exports_original_sysex() {
        let mut plugin = Rf106::default();
        let request = serde_json::to_vec(&ProgramEditRequest {
            schema_version: PROGRAM_EDIT_SCHEMA_VERSION,
            program_id: Some("factory.rf106.000".to_string()),
        })
        .unwrap();
        let mut transfer = vec![0; 64 * 1024];
        let length = plugin.begin_program_edit(&request, &mut transfer).unwrap();
        let prepared: PreparedProgram = serde_json::from_slice(&transfer[..length]).unwrap();
        assert_eq!(prepared.document.id, "user.rf106-001");
        assert_eq!(
            prepared.artifacts[0].storage_path,
            "programs/user-rf106-001.syx"
        );
        let encoded = serde_json::to_vec(&prepared).unwrap();
        assert!(plugin.install_program(&encoded));
        assert!(plugin.load_preset("custom.user.rf106-001"));
    }
}
