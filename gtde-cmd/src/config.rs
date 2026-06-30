use std::{collections::HashSet, path::PathBuf};

use serde_derive::{Deserialize, Serialize};

use crate::loadable::Loadable;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub profile_path: PathBuf,
    pub extra_dll_locations: Vec<PathBuf>,
    pub dev_dependencies: HashSet<String>,
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
