use alloc::{format, string::String, vec::Vec};
use rf_106_contract::sysex::Tone;

pub const CUSTOM_PREFIX: &str = "custom.";
pub const CASSETTE_PREFIX: &str = "cassette.rf106.";
pub const CASSETTE_BAYS: usize = 8;
pub const CASSETTE_RESOURCES: [&str; CASSETTE_BAYS] = [
    "cassette-1",
    "cassette-2",
    "cassette-3",
    "cassette-4",
    "cassette-5",
    "cassette-6",
    "cassette-7",
    "cassette-8",
];
pub const MAX_CUSTOM_PROGRAMS: usize = 128;
pub const MAX_PROGRAMS_PER_CASSETTE: usize = 128;
const ID_PREFIX: &str = "user.rf106-";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    pub id: String,
    pub name: String,
    pub tone: Tone,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CustomPrograms {
    entries: Vec<Program>,
}

impl CustomPrograms {
    pub fn entries(&self) -> &[Program] {
        &self.entries
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn document_id(catalog_id: &str) -> Option<&str> {
        catalog_id
            .strip_prefix(CUSTOM_PREFIX)
            .filter(|id| !id.is_empty())
    }

    pub fn find(&self, id: &str) -> Option<&Program> {
        self.entries.iter().find(|entry| entry.id == id)
    }

    pub fn install(&mut self, program: Program) -> bool {
        if let Some(slot) = self.entries.iter_mut().find(|entry| entry.id == program.id) {
            *slot = program;
            return true;
        }
        if self.entries.len() >= MAX_CUSTOM_PROGRAMS {
            return false;
        }
        self.entries.push(program);
        true
    }

    pub fn next_id(&self) -> String {
        (1..=MAX_CUSTOM_PROGRAMS + 1)
            .map(|number| format!("{ID_PREFIX}{number:03}"))
            .find(|candidate| self.find(candidate).is_none())
            .expect("there are more identifiers than custom program slots")
    }
}

pub fn cassette_bay(resource_id: &str) -> Option<usize> {
    CASSETTE_RESOURCES
        .iter()
        .position(|candidate| *candidate == resource_id)
}

pub fn cassette_program(id: &str) -> Option<(usize, usize)> {
    let suffix = id.strip_prefix(CASSETTE_PREFIX)?;
    let (bay, program) = suffix.split_once('.')?;
    if bay.len() != 1
        || !bay.bytes().all(|byte| byte.is_ascii_digit())
        || program.len() != 3
        || !program.bytes().all(|byte| byte.is_ascii_digit())
    {
        return None;
    }
    let bay = bay.parse::<usize>().ok()?.checked_sub(1)?;
    let program = program.parse::<usize>().ok()?;
    (bay < CASSETTE_BAYS && program < MAX_PROGRAMS_PER_CASSETTE).then_some((bay, program))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn program(id: &str) -> Program {
        Program {
            id: id.into(),
            name: id.into(),
            tone: Tone::new([0; 18]).unwrap(),
        }
    }

    #[test]
    fn custom_ids_and_replacement_are_stable() {
        let mut programs = CustomPrograms::default();
        assert_eq!(programs.next_id(), "user.rf106-001");
        assert!(programs.install(program("user.rf106-001")));
        assert!(programs.install(program("user.rf106-001")));
        assert_eq!(programs.entries().len(), 1);
        assert_eq!(programs.next_id(), "user.rf106-002");
        assert_eq!(
            CustomPrograms::document_id("custom.user.rf106-001"),
            Some("user.rf106-001")
        );
    }

    #[test]
    fn cassette_resources_and_program_ids_name_exact_bays() {
        assert_eq!(cassette_bay("cassette-1"), Some(0));
        assert_eq!(cassette_bay("cassette-8"), Some(7));
        assert_eq!(cassette_bay("cassette-9"), None);
        assert_eq!(cassette_program("cassette.rf106.1.000"), Some((0, 0)));
        assert_eq!(cassette_program("cassette.rf106.8.127"), Some((7, 127)));
        assert_eq!(cassette_program("cassette.rf106.8.128"), None);
        assert_eq!(cassette_program("cassette.rf106.0.000"), None);
    }
}
