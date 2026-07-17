use std::{fs, path::Path};

use gtde_error::loadable::LoadableError;
use serde::{Serialize, de::DeserializeOwned};

pub trait Loadable: Serialize + DeserializeOwned {
    fn load<P: AsRef<Path>>(env_path: P, file_name: &'static str) -> Result<Self, LoadableError> {
        let path = env_path.as_ref().join(file_name).with_extension("json");

        let data = fs::read_to_string(path).map_err(|e| LoadableError {
            error_type: e.into(),
            file_name: file_name,
            was_loading: true,
        })?;

        serde_json::from_str::<Self>(&data).map_err(|e| LoadableError {
            error_type: e.into(),
            file_name: file_name,
            was_loading: true,
        })
    }

    fn save(&self, env_path: &Path, file_name: &'static str) -> Result<(), LoadableError> {
        let data = serde_json::to_string_pretty::<Self>(self).map_err(|e| LoadableError {
            error_type: e.into(),
            file_name: file_name,
            was_loading: false,
        })?;

        fs::write(env_path.join(file_name).with_extension("json"), data).map_err(|e| LoadableError {
            error_type: e.into(),
            file_name: file_name,
            was_loading: false,
        })?;

        Ok(())
    }
}
