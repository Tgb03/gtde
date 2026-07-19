use std::path::Path;
use colored::Colorize;
use gtde_error::error::Error;
use serde_json::Value::{self};

use crate::commands::grab_db::PROJECT_DATABLOCKS;

fn resolve_datablock_bytes(
    env_path: impl AsRef<Path>,
    datablock_name: &str,
) -> Result<Vec<u8>, Error> {
    let plugins_dir = env_path.as_ref().join("plugins");
    let target_name = format!("GameData_{}DataBlock_bin.json", datablock_name).to_lowercase();

    // 1. Look on disk first
    let disk_match = std::fs::read_dir(&plugins_dir)
        .map_err(Error::io_at(&plugins_dir))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .find(|path| {
            path.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|name| name.to_lowercase() == target_name)
        });

    if let Some(path) = disk_match {
        return std::fs::read(&path).map_err(Error::io_at(&path));
    }

    // 2. Fall back to embedded resources
    PROJECT_DATABLOCKS
        .files()
        .find(|f| {
            f.path()
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|name| name.to_lowercase() == target_name)
        })
        .map(|f| f.contents().to_vec())
        .ok_or_else(|| Error::NoMatchingDataBlock(datablock_name.to_owned()))
}

pub fn search_db(env_path: impl AsRef<Path>, datablock_name: &str, id: u64, custom_field: Option<String>) -> Result<(), Error> {
    let mut number_of_found_objects = 0usize;

    let data = resolve_datablock_bytes(&env_path, datablock_name)?;
    let json_value = serde_json::from_slice(&data)?;
    let Value::Object(json_object) = json_value else {
        return Err(Error::InvalidJSONObject);
    };
    let Some(Value::Array(json_array)) = json_object.get("Blocks") else {
        return Err(Error::InvalidJSONObject);
    };

    for value in json_array {
        let Value::Object(object) = value else {
            continue;
        };
        let Some(id_val) = object.get("persistentID") else {
            continue;
        };
        let Value::Number(id_u32) = id_val else {
            continue;
        };

        if id_u32.as_u64().is_some_and(|e| e == id) {
            match &custom_field {
                Some(custom) => {
                    let Some(actual_obj) = value.as_object().map(|e| e.get(custom)).flatten() else { continue; };
                    
                    println!("{}", serde_json::to_string_pretty(actual_obj)?.green());
                    number_of_found_objects += 1;
                },
                None => {
                    println!("{}", serde_json::to_string_pretty(value)?.green());
                    number_of_found_objects += 1;
                },
            }
        }
    }

    println!("Number of total objects found: {}", number_of_found_objects);

    Ok(())
}
