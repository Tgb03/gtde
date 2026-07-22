use colored::Colorize;
use gtde_db::datablocks::datablock_wrapper::DatablockWrapper;
use gtde_error::error::Error;
use std::path::Path;

use crate::commands::grab_db::PROJECT_DATABLOCKS;
use crate::commands::init::DatablockEnum;

fn resolve_datablock_bytes<'a>(
    env_path: impl AsRef<Path>,
    datablock: DatablockEnum,
) -> Result<Vec<u8>, Error> {
    let plugins_dir = env_path.as_ref().join("plugins");
    let target_name = datablock.to_string();
    
    let disk_match = std::fs::read(&plugins_dir.join(&target_name))
        .ok()
        .or_else(|| {
            PROJECT_DATABLOCKS.get_file(&target_name)
                .map(|e| e.contents().to_owned())
        })
        .unwrap();

    Ok(disk_match)
}

pub fn search_db(
    env_path: impl AsRef<Path>,
    datablock: DatablockEnum,
    id: u32,
    custom_field: Option<String>,
) -> Result<(), Error> {
    let mut number_of_found_objects = 0usize;

    let data = resolve_datablock_bytes(&env_path, datablock)?;
    let json_value: DatablockWrapper<serde_json::Map<String, serde_json::Value>> = serde_json::from_slice(&data)?;

    for value in json_value.as_ref() {
        if value.persistent_id == id {
            if let Some(custom) = custom_field.as_ref() {
                println!("{}", serde_json::to_string_pretty(&value.data.get(custom))?.green());
                number_of_found_objects += 1;
                
                continue;
            }
            
            println!("{}", serde_json::to_string_pretty(&value.data)?.green());
            number_of_found_objects += 1;
        }
    }

    println!("Number of total objects found: {}", number_of_found_objects);

    Ok(())
}
