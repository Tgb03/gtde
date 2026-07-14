use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{loadable::Loadable, named_data::NamedData};

#[derive(Debug, Serialize, Deserialize)]
pub enum VersionType {
    Major,
    Minor,
    Patch,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub name: String,
    #[serde(with = "version_string")]
    pub version_number: (u8, u8, u8),
    pub website_url: String,
    pub description: String,
    pub dependencies: HashSet<String>,
}

mod version_string {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(version: &(u8, u8, u8), serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}.{}.{}", version.0, version.1, version.2);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<(u8, u8, u8), D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() != 3 {
            return Err(serde::de::Error::custom(format!(
                "invalid version string '{s}', expected format 'M.m.p'"
            )));
        }

        let parse = |p: &str| {
            p.parse::<u8>().map_err(|e| {
                serde::de::Error::custom(format!("invalid version component '{p}': {e}"))
            })
        };

        Ok((parse(parts[0])?, parse(parts[1])?, parse(parts[2])?))
    }
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            name: Default::default(),
            version_number: (0, 0, 1),
            website_url: Default::default(),
            description: Default::default(),
            dependencies: Default::default(),
        }
    }
}

impl NamedData for Manifest {
    fn get_name() -> &'static str {
        "manifest.json"
    }
}
impl Loadable for Manifest {}

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
