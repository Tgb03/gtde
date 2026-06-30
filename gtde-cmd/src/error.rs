use thiserror::Error;

use crate::loadable::LoadableError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No configuration file in project directory. Is there even a project here?")]
    NoConfig,
    #[error("No icon in directory. Please add \"icon.png\" to the project directory.")]
    NoIcon,
    #[error("No manifest in directory. Please add \"manifest.json\" to the project directory.")]
    NoManifest,
    #[error("Failed to copy a folder.")]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    LoadingError(#[from] LoadableError),
}
