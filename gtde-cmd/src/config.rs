use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::loadable::Loadable;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)] pub profile_path: PathBuf,
    #[serde(default)] pub extra_dll_locations: Vec<PathBuf>,
    #[serde(default)] pub dev_dependencies: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            profile_path: Default::default(),
            extra_dll_locations: Default::default(),
            dev_dependencies: Default::default(),
        }
    }
}

impl Loadable for Config {
    fn get_name() -> &'static str {
        "gtde.config"
    }
}
