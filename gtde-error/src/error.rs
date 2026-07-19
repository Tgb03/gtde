use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::loadable::LoadableError;

#[derive(Error, Debug)]
#[error(
    "Ran out persistentIDs. This is because you have 2^31 entries in this datablock. What happened?"
)]
pub struct ErrorRanOutOfPersistentIDs;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No configuration file in project directory. Is there even a project here?")]
    NoConfig,
    #[error("No icon in directory. Please add \"icon.png\" to the project directory.")]
    NoIcon,
    #[error("No manifest in directory. Please add \"manifest.json\" to the project directory.")]
    NoManifest,
    #[error("I/O error at '{path}': {source}")]
    IOErrorAt {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    LoadingError(#[from] LoadableError),
    #[error("No file found at path: {0}")]
    FileNotFound(String),
    #[error("Datablock {0} already exists. Cannot replace automatically.")]
    DatablockAlreadyExists(String),
    #[error("String {0} did not match any existing datablock.")]
    NoMatchingDataBlock(String),
    #[error("String {0} matched multiple datablocks: \n{1:?}")]
    AmbiguousDataBlock(String, Vec<String>),
    #[error("Constructor {0} did not exist. Created a default one in create/ folder.")]
    ConstructorDidNotExist(String),
    #[error(
        "JSON object has an invalid structure. Good luck. This error normally should not happen"
    )]
    InvalidJSONObject,
    #[error(transparent)]
    ErrorRanOutOfPersistentIDs(#[from] ErrorRanOutOfPersistentIDs),
}

impl Error {
    pub fn io_at(path: impl AsRef<Path>) -> impl FnOnce(std::io::Error) -> Error {
        let path = path.as_ref().to_owned();
        move |source| Error::IOErrorAt { path, source }
    }
}
