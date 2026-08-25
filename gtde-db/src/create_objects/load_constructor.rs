use std::path::Path;

use gtde_error::error::Error;
use schemars::{JsonSchema, schema_for};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

pub fn create_schema<C: JsonSchema>(env_path: impl AsRef<Path>, name: &str) -> Result<(), Error> {
    let schema = schema_for!(C);
    let schema_text = serde_json::to_string_pretty(&schema)?;
    let schema_path = env_path
        .as_ref()
        .join(".schemas")
        .join(format!("create-{}", name))
        .with_extension("json");
    std::fs::write(schema_path, schema_text)?;

    Ok(())
}

pub fn create_constructor(
    env_path: impl AsRef<Path>,
    name: &str,
    mut data: serde_json::Value,
) -> Result<(), Error> {
    let create_folder = env_path.as_ref().join("gtde-create");
    if let Value::Object(map) = &mut data {
        map.insert(
            "$schema".to_owned(),
            Value::String(format!("../.schemas/create-{}.json", name)),
        );
    }
    let data = serde_json::to_string_pretty(&data)?;
    let file_path = create_folder.join(name).with_extension("json");
    let _ = std::fs::write(&file_path, data)?;

    Ok(())
}

pub fn load_constructor<C: JsonSchema + Serialize + DeserializeOwned>(
    env_path: impl AsRef<Path>,
    name: &str,
) -> Result<C, Error> {
    let create_folder = env_path.as_ref().join("gtde-create");

    let file_path = create_folder.join(name).with_extension("json");
    let serialized_data = std::fs::read(&file_path).map_err(Error::io_at(&file_path))?;
    let object = serde_json::from_slice(&serialized_data)?;

    Ok(object)
}
