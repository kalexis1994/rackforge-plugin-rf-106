use alloc::{string::String, vec::Vec};
use rf_106_contract::sysex::{TONE_BYTES, Tone};

const MAGIC: &[u8; 6] = b"!j106\\";
const KIND: &str = "Library";
const MAJOR_VERSION: u32 = 3;
const SUPPORTED_MINOR_VERSIONS: core::ops::RangeInclusive<u32> = 0..=1;
const ENDING: [u8; 4] = [22, 87, 73, 106];
const LOOKUP: [u16; 16] = [
    34, 68, 40, 103, 61, 43, 49, 45, 71, 124, 6, 50, 46, 50, 32, 104,
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LibraryProgram {
    pub name: String,
    pub tone: Tone,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Library {
    pub name: String,
    pub programs: Vec<LibraryProgram>,
}

pub fn parse(bytes: &[u8], maximum_programs: usize) -> Option<Library> {
    let mut cursor = Cursor::new(bytes);
    cursor.expect(MAGIC)?;
    (cursor.string()? == KIND).then_some(())?;
    (cursor.u32()? == MAJOR_VERSION).then_some(())?;
    let minor = cursor.u32()?;
    SUPPORTED_MINOR_VERSIONS.contains(&minor).then_some(())?;
    let name = cursor.string()?;
    let count = usize::try_from(cursor.u64()?).ok()?;
    (count <= maximum_programs).then_some(())?;
    cursor.u64()?; // Library save time.
    cursor.verify_crc_from(0)?;

    let mut programs = Vec::with_capacity(count);
    for index in 0..count {
        let record_start = cursor.position();
        let name = cursor.string()?;
        if minor == 1 {
            cursor.string()?; // Stable UUID, not needed by RackForge.
        }
        cursor.u64()?; // Patch creation time.

        let key = (index + 79).checked_mul(137)?.checked_sub(31)?;
        let mut tone = [0_u8; TONE_BYTES];
        for (offset, byte) in tone.iter_mut().enumerate() {
            let encoded = cursor.u16()?;
            let divisor = LOOKUP[(offset + key) % LOOKUP.len()];
            (encoded % divisor == 0).then_some(())?;
            let decoded = encoded / divisor;
            (decoded <= 0x7f).then_some(())?;
            *byte = decoded as u8;
        }
        cursor.verify_crc_from(record_start)?;
        programs.push(LibraryProgram {
            name,
            tone: Tone::new(tone)?,
        });
    }

    cursor.expect(&ENDING)?;
    cursor.finished().then_some(Library { name, programs })
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn position(&self) -> usize {
        self.position
    }

    fn take(&mut self, length: usize) -> Option<&'a [u8]> {
        let end = self.position.checked_add(length)?;
        let value = self.bytes.get(self.position..end)?;
        self.position = end;
        Some(value)
    }

    fn expect(&mut self, expected: &[u8]) -> Option<()> {
        (self.take(expected.len())? == expected).then_some(())
    }

    fn u16(&mut self) -> Option<u16> {
        Some(u16::from_be_bytes(self.take(2)?.try_into().ok()?))
    }

    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_be_bytes(self.take(4)?.try_into().ok()?))
    }

    fn u64(&mut self) -> Option<u64> {
        Some(u64::from_be_bytes(self.take(8)?.try_into().ok()?))
    }

    fn string(&mut self) -> Option<String> {
        let length = usize::try_from(self.u32()?).ok()?;
        String::from_utf8(self.take(length)?.to_vec()).ok()
    }

    fn verify_crc_from(&mut self, start: usize) -> Option<()> {
        let end = self.position;
        let expected = self.u64()?;
        (expected == u64::from(crc32(self.bytes.get(start..end)?))).then_some(())
    }

    fn finished(&self) -> bool {
        self.position == self.bytes.len()
    }
}

fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = u32::MAX;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            let mask = 0_u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0xedb8_8320 & mask);
        }
    }
    !crc
}

#[cfg(test)]
mod tests {
    use super::*;
    fn push_u32(bytes: &mut Vec<u8>, value: u32) {
        bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn push_u64(bytes: &mut Vec<u8>, value: u64) {
        bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn push_string(bytes: &mut Vec<u8>, value: &str) {
        push_u32(bytes, value.len() as u32);
        bytes.extend_from_slice(value.as_bytes());
    }

    fn finish_crc(bytes: &mut Vec<u8>, start: usize) {
        let crc = crc32(&bytes[start..]);
        push_u64(bytes, u64::from(crc));
    }

    fn library(minor: u32, entries: &[(&str, [u8; TONE_BYTES])]) -> Vec<u8> {
        let mut bytes = MAGIC.to_vec();
        push_string(&mut bytes, KIND);
        push_u32(&mut bytes, MAJOR_VERSION);
        push_u32(&mut bytes, minor);
        push_string(&mut bytes, "RF-106 fixture");
        push_u64(&mut bytes, entries.len() as u64);
        push_u64(&mut bytes, 1_700_000_000_000);
        finish_crc(&mut bytes, 0);

        for (index, (name, tone)) in entries.iter().enumerate() {
            let start = bytes.len();
            push_string(&mut bytes, name);
            if minor == 1 {
                push_string(&mut bytes, "00000000-0000-0000-0000-000000000000");
            }
            push_u64(&mut bytes, 1_700_000_000_000 + index as u64);
            let key = (index + 79) * 137 - 31;
            for (offset, byte) in tone.iter().enumerate() {
                let encoded = u16::from(*byte) * LOOKUP[(offset + key) % LOOKUP.len()];
                bytes.extend_from_slice(&encoded.to_be_bytes());
            }
            finish_crc(&mut bytes, start);
        }
        bytes.extend_from_slice(&ENDING);
        bytes
    }

    #[test]
    fn current_librarian_files_retain_names_and_exact_tone_bytes() {
        let first = core::array::from_fn(|index| index as u8);
        let second = core::array::from_fn(|index| 127 - index as u8);
        let bytes = library(1, &[("A-11 Bass", first), ("B-88 Strings", second)]);
        let library = parse(&bytes, 128).unwrap();
        assert_eq!(library.name, "RF-106 fixture");
        assert_eq!(library.programs.len(), 2);
        assert_eq!(library.programs[0].name, "A-11 Bass");
        assert_eq!(library.programs[0].tone.bytes(), &first);
        assert_eq!(library.programs[1].name, "B-88 Strings");
        assert_eq!(library.programs[1].tone.bytes(), &second);
    }

    #[test]
    fn version_three_without_patch_uuids_is_supported() {
        let bytes = library(0, &[("Legacy", [64; TONE_BYTES])]);
        assert_eq!(parse(&bytes, 128).unwrap().programs[0].name, "Legacy");
    }

    #[test]
    fn corruption_unknown_versions_and_oversized_libraries_are_rejected() {
        let mut bytes = library(1, &[("Patch", [0; TONE_BYTES])]);
        bytes[42] ^= 1;
        assert!(parse(&bytes, 128).is_none());

        let mut bytes = library(1, &[("Patch", [0; TONE_BYTES])]);
        bytes[21..25].copy_from_slice(&4_u32.to_be_bytes());
        assert!(parse(&bytes, 128).is_none());

        let bytes = library(1, &[("Patch", [0; TONE_BYTES])]);
        assert!(parse(&bytes, 0).is_none());
    }

    #[test]
    fn java_crc32_matches_the_standard_check_vector() {
        assert_eq!(crc32(b"123456789"), 0xcbf4_3926);
    }
}
