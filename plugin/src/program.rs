use alloc::{format, string::String, string::ToString, vec, vec::Vec};
use rf_106_contract::{
    PLUGIN_ID, STATE_SCHEMA_VERSION,
    sysex::{APR_MANUAL, APR_PROGRAM, Tone},
};
use serde::{Deserialize, Serialize};

use crate::programs::{CUSTOM_PREFIX, CustomPrograms, Program};

pub const PROGRAM_SCHEMA_VERSION: u32 = 1;
pub const PROGRAM_EDIT_SCHEMA_VERSION: u32 = 1;
pub const PROGRAM_EDITOR_SCHEMA_VERSION: u32 = 1;
pub const PAYLOAD_VERSION: u32 = 1;
const PAYLOAD_FORMAT: &str = "roland-juno-106-tone";
const SYSEX_MEDIA_TYPE: &str = "audio/x-midi";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TonePayload {
    pub format: String,
    pub version: u32,
    pub bytes: Vec<u8>,
}

impl TonePayload {
    fn from_tone(tone: Tone) -> Self {
        Self {
            format: PAYLOAD_FORMAT.to_string(),
            version: PAYLOAD_VERSION,
            bytes: tone.bytes().to_vec(),
        }
    }

    fn to_tone(&self) -> Option<Tone> {
        if self.format != PAYLOAD_FORMAT || self.version != PAYLOAD_VERSION {
            return None;
        }
        Tone::new(self.bytes.as_slice().try_into().ok()?)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProgramDocument {
    pub schema_version: u32,
    pub id: String,
    pub name: String,
    pub plugin_id: String,
    pub plugin_version: String,
    pub plugin_state_version: u32,
    pub payload_version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    pub payload: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProgramEditRequest {
    pub schema_version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub program_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProgramArtifact {
    pub storage_path: String,
    pub media_type: String,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreparedProgram {
    pub schema_version: u32,
    pub storage_path: String,
    pub preview_sound_id: String,
    pub document: ProgramDocument,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<ProgramArtifact>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum EditorValue {
    Inherited,
    Boolean(bool),
    Integer(i64),
    Choice(String),
    SoundId(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProgramFieldEditRequest {
    pub schema_version: u32,
    pub document: ProgramDocument,
    pub field_id: String,
    pub value: EditorValue,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditorChoice {
    pub value: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EditorFieldKind {
    Toggle,
    Number {
        minimum: i64,
        maximum: i64,
        step: i64,
        decimals: u8,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
        allow_inherited: bool,
    },
    Choice {
        options: Vec<EditorChoice>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditorField {
    pub id: String,
    pub label: String,
    pub detail: String,
    pub value: EditorValue,
    pub kind: EditorFieldKind,
    pub live_preview: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditorPage {
    pub id: String,
    pub label: String,
    pub detail: String,
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pages: Vec<EditorPage>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<EditorField>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditorView {
    pub schema_version: u32,
    pub title: String,
    pub pages: Vec<EditorPage>,
}

pub fn document(id: &str, name: &str, tone: Tone) -> ProgramDocument {
    ProgramDocument {
        schema_version: PROGRAM_SCHEMA_VERSION,
        id: id.to_string(),
        name: clean_name(name),
        plugin_id: PLUGIN_ID.to_string(),
        plugin_version: env!("CARGO_PKG_VERSION").to_string(),
        plugin_state_version: STATE_SCHEMA_VERSION,
        payload_version: PAYLOAD_VERSION,
        category: Some("Analog".to_string()),
        tags: vec!["JUNO-106".to_string(), "SysEx".to_string()],
        payload: serde_json::to_value(TonePayload::from_tone(tone)).expect("plain tone payload"),
    }
}

pub fn tone_of(document: &ProgramDocument) -> Option<Tone> {
    if document.schema_version != PROGRAM_SCHEMA_VERSION
        || document.plugin_id != PLUGIN_ID
        || document.payload_version != PAYLOAD_VERSION
    {
        return None;
    }
    serde_json::from_value::<TonePayload>(document.payload.clone())
        .ok()?
        .to_tone()
}

pub fn prepared(document: ProgramDocument, custom: &CustomPrograms) -> Option<PreparedProgram> {
    let tone = tone_of(&document)?;
    let stem = document.id.replace('.', "-");
    let mut saved: Vec<Tone> = custom
        .entries()
        .iter()
        .map(|program| program.tone)
        .collect();
    match custom
        .entries()
        .iter()
        .position(|program| program.id == document.id)
    {
        Some(index) => saved[index] = tone,
        None => saved.push(tone),
    }
    let mut artifacts = vec![ProgramArtifact {
        storage_path: format!("programs/{stem}.syx"),
        media_type: SYSEX_MEDIA_TYPE.to_string(),
        bytes: tone.encode_apr(APR_MANUAL, 0, 0)?.to_vec(),
    }];
    artifacts.push(ProgramArtifact {
        storage_path: "exports/rf106-programs.syx".to_string(),
        media_type: SYSEX_MEDIA_TYPE.to_string(),
        bytes: bank_bytes(&saved),
    });
    let factory = (0..rf_106_contract::FACTORY_PRESET_COUNT as u32)
        .filter_map(rf_106_contract::factory_preset)
        .map(Tone::from_factory)
        .collect::<Vec<_>>();
    artifacts.push(ProgramArtifact {
        storage_path: "exports/rf106-factory.syx".to_string(),
        media_type: SYSEX_MEDIA_TYPE.to_string(),
        bytes: bank_bytes(&factory),
    });
    Some(PreparedProgram {
        schema_version: PROGRAM_EDIT_SCHEMA_VERSION,
        storage_path: format!("programs/{stem}.json"),
        preview_sound_id: format!("{CUSTOM_PREFIX}{}", document.id),
        document,
        artifacts,
    })
}

pub fn program_of(document: &ProgramDocument) -> Option<Program> {
    Some(Program {
        id: document.id.clone(),
        name: clean_name(&document.name),
        tone: tone_of(document)?,
    })
}

pub fn editor_view(document: &ProgramDocument) -> Option<EditorView> {
    let tone = tone_of(document)?;
    let bytes = tone.bytes();
    let number = |id: &str, label: &str, slot: usize, maximum: i64| EditorField {
        id: id.to_string(),
        label: label.to_string(),
        detail: "Original JUNO-106 seven-bit tone value.".to_string(),
        value: EditorValue::Integer(i64::from(bytes[slot])),
        kind: EditorFieldKind::Number {
            minimum: 0,
            maximum,
            step: 1,
            decimals: 0,
            unit: None,
            allow_inherited: false,
        },
        live_preview: true,
    };
    let toggle = |id: &str, label: &str, value: bool| EditorField {
        id: id.to_string(),
        label: label.to_string(),
        detail: "Original front-panel switch.".to_string(),
        value: EditorValue::Boolean(value),
        kind: EditorFieldKind::Toggle,
        live_preview: true,
    };
    let choice = |id: &str, label: &str, value: &str, values: &[(&str, &str)]| EditorField {
        id: id.to_string(),
        label: label.to_string(),
        detail: "Original front-panel switch.".to_string(),
        value: EditorValue::Choice(value.to_string()),
        kind: EditorFieldKind::Choice {
            options: values
                .iter()
                .map(|(value, label)| EditorChoice {
                    value: (*value).to_string(),
                    label: (*label).to_string(),
                    detail: None,
                })
                .collect(),
        },
        live_preview: true,
    };

    let names = [
        ("lfo-rate", "LFO Rate", 0),
        ("lfo-delay", "LFO Delay", 1),
        ("dco-lfo", "DCO LFO", 2),
        ("dco-pwm", "DCO PWM", 3),
        ("dco-noise", "Noise", 4),
        ("vcf-cutoff", "VCF Cutoff", 5),
        ("vcf-resonance", "VCF Resonance", 6),
        ("vcf-envelope", "VCF Envelope", 7),
        ("vcf-lfo", "VCF LFO", 8),
        ("vcf-keyboard", "VCF Keyboard", 9),
        ("vca-level", "VCA Level", 10),
        ("env-attack", "Attack", 11),
        ("env-decay", "Decay", 12),
        ("env-sustain", "Sustain", 13),
        ("env-release", "Release", 14),
        ("dco-sub", "Sub", 15),
    ];
    let mut fields = names
        .iter()
        .map(|(id, label, slot)| number(id, label, *slot, if *slot == 3 { 105 } else { 127 }))
        .collect::<Vec<_>>();
    let sw1 = bytes[16];
    let sw2 = bytes[17];
    fields.extend([
        choice(
            "dco-range",
            "Range",
            if sw1 & 4 != 0 {
                "4"
            } else if sw1 & 2 != 0 {
                "8"
            } else {
                "16"
            },
            &[("16", "16'"), ("8", "8'"), ("4", "4'")],
        ),
        toggle("dco-pulse", "Pulse", sw1 & 0x08 != 0),
        toggle("dco-saw", "Saw", sw1 & 0x10 != 0),
        choice(
            "chorus",
            "Chorus",
            if sw1 & 0x20 != 0 {
                "off"
            } else if sw1 & 0x40 != 0 {
                "i"
            } else {
                "ii"
            },
            &[("off", "Off"), ("i", "I"), ("ii", "II")],
        ),
        choice(
            "hpf",
            "HPF",
            &format!("{}", 3 - ((sw2 >> 3) & 3)),
            &[("0", "0"), ("1", "1"), ("2", "2"), ("3", "3")],
        ),
        choice(
            "pwm-mode",
            "PWM Mode",
            if sw2 & 1 != 0 { "manual" } else { "lfo" },
            &[("lfo", "LFO"), ("manual", "Manual")],
        ),
        choice(
            "vcf-polarity",
            "VCF Polarity",
            if sw2 & 2 != 0 { "negative" } else { "positive" },
            &[("positive", "+"), ("negative", "-")],
        ),
        choice(
            "vca-mode",
            "VCA Mode",
            if sw2 & 4 != 0 { "gate" } else { "env" },
            &[("env", "Envelope"), ("gate", "Gate")],
        ),
    ]);
    Some(EditorView {
        schema_version: PROGRAM_EDITOR_SCHEMA_VERSION,
        title: "JUNO-106 Tone".to_string(),
        pages: vec![EditorPage {
            id: "tone".to_string(),
            label: "Tone".to_string(),
            detail: "The original eighteen-byte JUNO-106 program memory.".to_string(),
            enabled: true,
            pages: Vec::new(),
            fields,
        }],
    })
}

pub fn apply_edit(request: ProgramFieldEditRequest) -> Option<ProgramDocument> {
    if request.schema_version != PROGRAM_EDIT_SCHEMA_VERSION {
        return None;
    }
    let tone = tone_of(&request.document)?;
    let mut bytes = *tone.bytes();
    let continuous = [
        "lfo-rate",
        "lfo-delay",
        "dco-lfo",
        "dco-pwm",
        "dco-noise",
        "vcf-cutoff",
        "vcf-resonance",
        "vcf-envelope",
        "vcf-lfo",
        "vcf-keyboard",
        "vca-level",
        "env-attack",
        "env-decay",
        "env-sustain",
        "env-release",
        "dco-sub",
    ];
    if let Some(slot) = continuous.iter().position(|id| *id == request.field_id) {
        let EditorValue::Integer(value) = request.value else {
            return None;
        };
        let maximum = if slot == 3 { 105 } else { 127 };
        bytes[slot] = value.clamp(0, maximum) as u8;
    } else {
        match (request.field_id.as_str(), request.value) {
            ("dco-range", EditorValue::Choice(value)) => {
                bytes[16] = (bytes[16] & !0x07)
                    | match value.as_str() {
                        "4" => 4,
                        "8" => 2,
                        "16" => 1,
                        _ => return None,
                    };
            }
            ("dco-pulse", EditorValue::Boolean(value)) => set_bit(&mut bytes[16], 0x08, value),
            ("dco-saw", EditorValue::Boolean(value)) => set_bit(&mut bytes[16], 0x10, value),
            ("chorus", EditorValue::Choice(value)) => {
                bytes[16] &= !(0x20 | 0x40);
                match value.as_str() {
                    "off" => bytes[16] |= 0x20,
                    "i" => bytes[16] |= 0x40,
                    "ii" => {}
                    _ => return None,
                }
            }
            ("hpf", EditorValue::Choice(value)) => {
                let hpf: u8 = value.parse().ok()?;
                if hpf > 3 {
                    return None;
                }
                bytes[17] = (bytes[17] & !0x18) | ((3 - hpf) << 3);
            }
            ("pwm-mode", EditorValue::Choice(value)) => set_bit(
                &mut bytes[17],
                1,
                match value.as_str() {
                    "manual" => true,
                    "lfo" => false,
                    _ => return None,
                },
            ),
            ("vcf-polarity", EditorValue::Choice(value)) => set_bit(
                &mut bytes[17],
                2,
                match value.as_str() {
                    "negative" => true,
                    "positive" => false,
                    _ => return None,
                },
            ),
            ("vca-mode", EditorValue::Choice(value)) => set_bit(
                &mut bytes[17],
                4,
                match value.as_str() {
                    "gate" => true,
                    "env" => false,
                    _ => return None,
                },
            ),
            _ => return None,
        }
    }
    let tone = Tone::new(bytes)?;
    Some(document(&request.document.id, &request.document.name, tone))
}

fn set_bit(byte: &mut u8, mask: u8, enabled: bool) {
    if enabled {
        *byte |= mask;
    } else {
        *byte &= !mask;
    }
}

fn clean_name(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        "RF NEW".to_string()
    } else {
        trimmed.to_string()
    }
}

fn bank_bytes(tones: &[Tone]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(tones.len() * 24);
    for (index, tone) in tones.iter().copied().take(128).enumerate() {
        if let Some(message) = tone.encode_apr(APR_PROGRAM, 0, index as u8) {
            bytes.extend_from_slice(&message);
        }
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;
    use rf_106_contract::factory_preset;

    #[test]
    fn document_and_artifacts_keep_exact_original_tone_bytes() {
        let tone = Tone::from_factory(factory_preset(0).unwrap());
        let document = document("user.rf106-001", "Brass", tone);
        assert_eq!(tone_of(&document), Some(tone));
        let prepared = prepared(document, &CustomPrograms::default()).unwrap();
        assert_eq!(prepared.artifacts.len(), 3);
        assert_eq!(
            Tone::decode_apr(&prepared.artifacts[0].bytes).unwrap().tone,
            tone
        );
        assert_eq!(prepared.artifacts[1].bytes.len(), 24);
        assert_eq!(prepared.artifacts[2].bytes.len(), 128 * 24);
    }

    #[test]
    fn declarative_edits_update_the_original_bytes() {
        let tone = Tone::from_factory(factory_preset(0).unwrap());
        let document = document("user.rf106-001", "Brass", tone);
        let edited = apply_edit(ProgramFieldEditRequest {
            schema_version: PROGRAM_EDIT_SCHEMA_VERSION,
            document,
            field_id: "dco-pwm".to_string(),
            value: EditorValue::Integer(105),
        })
        .unwrap();
        assert_eq!(tone_of(&edited).unwrap().bytes()[3], 105);
        assert!(editor_view(&edited).unwrap().pages[0].fields.len() >= 24);
    }
}
