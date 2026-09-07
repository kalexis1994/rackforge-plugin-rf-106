use alloc::{string::ToString, vec, vec::Vec};
use serde::Serialize;

use crate::programs::{CUSTOM_PREFIX, CustomPrograms, IMPORTED_PREFIX, Program};

#[derive(Serialize)]
struct Catalog {
    schema_version: u32,
    banks: Vec<Bank>,
    presets: Vec<Entry>,
}

#[derive(Serialize)]
struct Bank {
    id: &'static str,
    name: &'static str,
    order: usize,
}

#[derive(Serialize)]
struct Entry {
    id: alloc::string::String,
    name: alloc::string::String,
    bank: &'static str,
    category: &'static str,
    editable: bool,
    order: usize,
}

pub fn write(
    imported: &[Program],
    custom: &CustomPrograms,
    destination: &mut [u8],
) -> Option<usize> {
    let mut banks = vec![Bank {
        id: "factory.rf106",
        name: "Original factory",
        order: 0,
    }];
    if !imported.is_empty() {
        banks.push(Bank {
            id: "imported.rf106",
            name: "Imported SysEx",
            order: 1,
        });
    }
    if !custom.is_empty() {
        banks.push(Bank {
            id: "user.rf106",
            name: "Your programs",
            order: 2,
        });
    }
    let mut presets = Vec::with_capacity(128 + imported.len() + custom.entries().len());
    for index in 0..rf_106_contract::FACTORY_PRESET_COUNT as u32 {
        let preset = rf_106_contract::factory_preset(index)?;
        presets.push(Entry {
            id: alloc::format!("factory.rf106.{index:03}"),
            name: preset.name.to_string(),
            bank: "factory.rf106",
            category: "Analog",
            editable: true,
            order: index as usize,
        });
    }
    for (index, program) in imported.iter().enumerate() {
        presets.push(Entry {
            id: alloc::format!("{IMPORTED_PREFIX}{index:03}"),
            name: program.name.clone(),
            bank: "imported.rf106",
            category: "Analog",
            editable: true,
            order: 128 + index,
        });
    }
    for (index, program) in custom.entries().iter().enumerate() {
        presets.push(Entry {
            id: alloc::format!("{CUSTOM_PREFIX}{}", program.id),
            name: program.name.clone(),
            bank: "user.rf106",
            category: "Analog",
            editable: true,
            order: 128 + imported.len() + index,
        });
    }
    let bytes = serde_json::to_vec(&Catalog {
        schema_version: 1,
        banks,
        presets,
    })
    .ok()?;
    let output = destination.get_mut(..bytes.len())?;
    output.copy_from_slice(&bytes);
    Some(bytes.len())
}
