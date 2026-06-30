use std::collections::HashSet;

use serde_derive::{Deserialize, Serialize};

use crate::loadable::Loadable;

#[derive(Debug, Serialize, Deserialize)]
pub enum VersionType {
    Major,
    Minor,
    Patch,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub name: String,
    pub author_name: String,
    pub version_number: (u8, u8, u8),
    pub website_url: String,
    pub description: String,
    pub dependencies: HashSet<String>,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            name: Default::default(),
            author_name: Default::default(),
            version_number: (0, 0, 1),
            website_url: Default::default(),
            description: Default::default(),
            dependencies: Default::default(),
        }
    }
}

impl Loadable for Manifest {
    fn get_name() -> &'static str {
        "manifest.json"
    }
}

impl Manifest {
    pub fn up_version(&mut self, version_number: VersionType) {
        self.version_number = match version_number {
            VersionType::Major => (self.version_number.0 + 1, 0, 0),
            VersionType::Minor => (self.version_number.0, self.version_number.1 + 1, 0),
            VersionType::Patch => (
                self.version_number.0,
                self.version_number.1,
                self.version_number.2 + 1,
            ),
        };
    }
}
