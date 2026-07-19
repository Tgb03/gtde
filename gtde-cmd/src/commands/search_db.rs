use std::path::{Path, PathBuf};
use colored::Colorize;
use gtde_error::error::Error;
use serde_json::Value::{self};

fn resolve_datablock_path(
    env_path: impl AsRef<Path>,
    datablock_name: &str,
) -> Result<PathBuf, Error> {
    let plugins_dir = env_path.as_ref().join("plugins");

    let needle = datablock_name.to_lowercase();

    let mut matches: Vec<PathBuf> = std::fs::read_dir(&plugins_dir)
        .map_err(Error::io_at(&plugins_dir))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension().is_some_and(|ext| ext == "json")
                && path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .is_some_and(|name| name.to_lowercase().contains(&needle))
        })
        .collect();

    match matches.len() {
        0 => Err(Error::NoMatchingDataBlock(datablock_name.to_owned())),
        1 => Ok(matches.remove(0)),
        _ => {
            let names: Vec<String> = matches
                .iter()
                .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(str::to_owned))
                .collect();
            Err(Error::AmbiguousDataBlock(datablock_name.to_owned(), names))
        }
    }
}

pub fn search_db(env_path: impl AsRef<Path>, datablock_name: &str, id: u64) -> Result<(), Error> {
    let mut number_of_found_objects = 0usize;

    let file_path = resolve_datablock_path(&env_path, datablock_name)?;
    let data = std::fs::read(&file_path).map_err(Error::io_at(&file_path))?;
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
            println!("{}", serde_json::to_string_pretty(value)?.green());
            number_of_found_objects += 1;
        }
    }

    println!("Number of total objects found: {}", number_of_found_objects);

    Ok(())
}
