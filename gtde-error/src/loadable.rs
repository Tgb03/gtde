use std::{fmt::Display, io};

use thiserror::Error;

#[derive(Debug, Error)]
pub struct LoadableError {
    pub error_type: LoadableErrorType,
    pub file_name: &'static str,
    pub was_loading: bool,
}

impl Display for LoadableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "error {} '{}': {}",
            match self.was_loading {
                true => "loading",
                false => "saving",
            },
            self.file_name,
            self.error_type
        )
    }
}

#[derive(Debug, Error)]
pub enum LoadableErrorType {
    #[error(transparent)]
    IOError(#[from] io::Error),
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
}
