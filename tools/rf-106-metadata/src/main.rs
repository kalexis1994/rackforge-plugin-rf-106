use rf_106_contract::{FACTORY_PRESET_COUNT, factory_preset};
use serde_json::json;
use std::{env, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("plugin/package/metadata/presets.json"));
    let presets = (0..FACTORY_PRESET_COUNT as u32)
        .map(|index| {
            let preset = factory_preset(index).expect("factory index has preset");
            json!({
                "id": format!("factory.rf106.{index:03}"),
                "name": preset.name,
                "bank": "factory.rf106",
                "order": index,
                "tags": ["rf106"],
                "editable": false
            })
        })
        .collect::<Vec<_>>();
    let catalog = json!({
        "schema_version": 1,
        "banks": [
            { "id": "factory.rf106", "name": "RF-106", "order": 0 }
        ],
        "presets": presets
    });
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output, serde_json::to_vec_pretty(&catalog)?)?;
    println!(
        "METADATA_WRITTEN path={} presets={}",
        output.display(),
        presets.len()
    );
    Ok(())
}
