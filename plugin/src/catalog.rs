use alloc::{string::ToString, vec, vec::Vec};
use serde::Serialize;

use crate::programs::{CASSETTE_BAYS, CASSETTE_PREFIX, CUSTOM_PREFIX, CustomPrograms, Program};

#[derive(Serialize)]
struct Catalog {
    schema_version: u32,
    banks: Vec<Bank>,
    presets: Vec<Entry>,
}

#[derive(Serialize)]
struct Bank {
    id: alloc::string::String,
    name: alloc::string::String,
    order: usize,
}

#[derive(Serialize)]
struct Entry {
    id: alloc::string::String,
    name: alloc::string::String,
    bank: alloc::string::String,
    category: &'static str,
    editable: bool,
    order: usize,
}

pub fn write(
    cassettes: &[Vec<Program>; CASSETTE_BAYS],
    cassette_names: &[Option<alloc::string::String>; CASSETTE_BAYS],
    custom: &CustomPrograms,
    destination: &mut [u8],
) -> Option<usize> {
    let mut banks = vec![Bank {
        id: "factory.rf106".to_string(),
        name: "Original factory".to_string(),
        order: 0,
    }];
    for (bay, programs) in cassettes.iter().enumerate() {
        if !programs.is_empty() {
            banks.push(Bank {
                id: alloc::format!("{CASSETTE_PREFIX}{}", bay + 1),
                name: cassette_names[bay]
                    .clone()
                    .unwrap_or_else(|| alloc::format!("Cassette {}", bay + 1)),
                order: bay + 1,
            });
        }
    }
    if !custom.is_empty() {
        banks.push(Bank {
            id: "user.rf106".to_string(),
            name: "Your programs".to_string(),
            order: CASSETTE_BAYS + 1,
        });
    }
    let imported_count = cassettes.iter().map(Vec::len).sum::<usize>();
    let mut presets = Vec::with_capacity(128 + imported_count + custom.entries().len());
    for index in 0..rf_106_contract::FACTORY_PRESET_COUNT as u32 {
        let preset = rf_106_contract::factory_preset(index)?;
        presets.push(Entry {
            id: alloc::format!("factory.rf106.{index:03}"),
            name: preset.name.to_string(),
            bank: "factory.rf106".to_string(),
            category: "Analog",
            editable: true,
            order: index as usize,
        });
    }
    let mut imported_order = 0;
    for (bay, programs) in cassettes.iter().enumerate() {
        let bank = alloc::format!("{CASSETTE_PREFIX}{}", bay + 1);
        for (index, program) in programs.iter().enumerate() {
            presets.push(Entry {
                id: alloc::format!("{bank}.{index:03}"),
                name: program.name.clone(),
                bank: bank.clone(),
                category: "Analog",
                editable: true,
                order: 128 + imported_order,
            });
            imported_order += 1;
        }
    }
    for (index, program) in custom.entries().iter().enumerate() {
        presets.push(Entry {
            id: alloc::format!("{CUSTOM_PREFIX}{}", program.id),
            name: program.name.clone(),
            bank: "user.rf106".to_string(),
            category: "Analog",
            editable: true,
            order: 128 + imported_count + index,
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
