use alloc::{format, string::String, vec::Vec};
use rf_106_contract::sysex::Tone;

pub const CUSTOM_PREFIX: &str = "custom.";
pub const IMPORTED_PREFIX: &str = "imported.rf106.";
pub const MAX_CUSTOM_PROGRAMS: usize = 128;
pub const MAX_IMPORTED_PROGRAMS: usize = 128;
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

pub fn imported_index(id: &str) -> Option<usize> {
    let digits = id.strip_prefix(IMPORTED_PREFIX)?;
    if digits.len() != 3 || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let index: usize = digits.parse().ok()?;
    (index < MAX_IMPORTED_PROGRAMS).then_some(index)
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
}
