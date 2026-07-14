use std::path::PathBuf;

use gtde_file::{loadable::Loadable, named_data::NamedData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub profile_path: PathBuf,
    #[serde(default)]
    pub extra_dll_locations: Vec<PathBuf>,
    #[serde(default)]
    pub dev_dependencies: Vec<String>,
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

impl NamedData for Config {
    fn get_name() -> &'static str {
        "gtde.config"
    }
}
impl Loadable for Config {}
