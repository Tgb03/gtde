use std::{fs, path::Path};

use gtde_error::loadable::LoadableError;
use serde::{Serialize, de::DeserializeOwned};

use crate::named_data::NamedData;

pub trait Loadable: NamedData + Serialize + DeserializeOwned {
    fn load<P: AsRef<Path>>(env_path: P) -> Result<Self, LoadableError> {
        let path = env_path.as_ref().join(Self::get_name());

        let data = fs::read_to_string(path).map_err(|e| LoadableError {
            error_type: e.into(),
            file_name: Self::get_name(),
            was_loading: true,
        })?;

        serde_json::from_str::<Self>(&data).map_err(|e| LoadableError {
            error_type: e.into(),
            file_name: Self::get_name(),
            was_loading: true,
        })
    }

    fn save(&self, env_path: &Path) -> Result<(), LoadableError> {
        let data = serde_json::to_string_pretty::<Self>(self).map_err(|e| LoadableError {
            error_type: e.into(),
            file_name: Self::get_name(),
            was_loading: false,
        })?;

        fs::write(env_path.join(Self::get_name()), data).map_err(|e| LoadableError {
            error_type: e.into(),
            file_name: Self::get_name(),
            was_loading: false,
        })?;

        Ok(())
    }
}
