use std::{fmt::Display, fs, io, path::Path};

use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;

#[derive(Debug, Error)]
pub struct LoadableError {
    error_type: LoadableErrorType,
    file_name: &'static str,
    was_loading: bool,
}

impl Display for LoadableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error {} '{}': {}", match self.was_loading {
            true => "loading",
            false => "saving",
        }, self.file_name, self.error_type)
    }
}

#[derive(Debug, Error)]
pub enum LoadableErrorType {
    #[error(transparent)]
    IOError(#[from] io::Error),
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
}

pub trait Loadable: Serialize + DeserializeOwned {
    fn get_name() -> &'static str;

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
