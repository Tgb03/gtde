use colored::Colorize;
use std::path::Path;

use gtde_error::error::Error;
use gtde_file::file_utils::{FileStatus, create_file_if_doesnt_exist};
use schemars::{JsonSchema, schema_for};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

fn create_if_not_existing<T: Serialize + JsonSchema>(
    path: impl AsRef<Path>,
    file_name: &str,
    data: T,
) -> Result<FileStatus, Error> {
    let mut data = serde_json::to_value(&data)?;
    if let Value::Object(map) = &mut data {
        map.insert(
            "$schema".to_owned(),
            Value::String(format!("../.schemas/create-{}", file_name)),
        );
    }
    let data = serde_json::to_string_pretty(&data)?;
    let status = create_file_if_doesnt_exist(path.as_ref().join("gtde-create"), file_name, data)?;

    if status == FileStatus::FileCreated {
        let schema = schema_for!(T);
        let schema_text = serde_json::to_string_pretty(&schema)?;
        let schema_path = path.as_ref().join(".schemas");
        create_file_if_doesnt_exist(schema_path, &format!("create-{}", file_name), schema_text)?;

        println!(
            "Created {}. You can now edit this and use the same command to inject the data into the datablocks",
            file_name.green()
        );
    }

    Ok(status)
}

pub fn create_constructor<C: Default + JsonSchema + Serialize>(
    env_path: impl AsRef<Path>,
    name: &str,
) -> Result<FileStatus, Error> {
    create_if_not_existing(&env_path, name, C::default())
}

pub fn load_constructor<C: Default + JsonSchema + Serialize + DeserializeOwned>(
    env_path: impl AsRef<Path>,
    name: &str,
) -> Result<C, Error> {
    let create_folder = env_path.as_ref().join("gtde-create");

    if FileStatus::FileCreated == create_constructor::<C>(&env_path, name)? {
        return Err(Error::ConstructorDidNotExist(name.to_owned()));
    }

    let file_path = create_folder.join(name);
    let serialized_data = std::fs::read(&file_path).map_err(Error::io_at(&file_path))?;
    let object = serde_json::from_slice(&serialized_data)?;

    Ok(object)
}
