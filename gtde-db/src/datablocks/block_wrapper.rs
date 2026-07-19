use gtde_file::named_data::NamedData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BlockWrapper<T> {
    #[serde(flatten)]
    pub data: T,

    #[serde(rename = "persistentID")]
    pub persistent_id: u32,
    #[serde(rename = "internalEnabled")]
    pub internal_enabled: bool,
    pub name: String,
}

impl<T: NamedData> NamedData for BlockWrapper<T> {
    fn get_name() -> &'static str {
        T::get_name()
    }
}

impl<T: PartialEq> PartialEq<T> for BlockWrapper<T> {
    fn eq(&self, other: &T) -> bool {
        &self.data == other
    }
}

impl<T: Default> Default for BlockWrapper<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            persistent_id: Default::default(),
            internal_enabled: Default::default(),
            name: Default::default(),
        }
    }
}

impl<T> BlockWrapper<T> {
    pub fn new(data: T, name: String, id: u32) -> Self {
        Self {
            data,
            persistent_id: id,
            internal_enabled: true,
            name,
        }
    }
}

impl<T> AsRef<T> for BlockWrapper<T> {
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T> AsMut<T> for BlockWrapper<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.data
    }
}
