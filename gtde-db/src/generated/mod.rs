// pub mod schemas;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::datablocks::reference::Reference;

pub mod chained_puzzle;
pub mod enemy;
pub mod enums;
pub mod survival_wave_population;
pub mod survival_wave_settings;
pub mod text;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum EnumWrapper<T> {
    Data(T),
    Int(u32),
}

impl<T: Default> Default for EnumWrapper<T> {
    fn default() -> Self {
        Self::Data(T::default())
    }
}

impl<T> From<T> for EnumWrapper<T> {
    fn from(value: T) -> Self {
        EnumWrapper::Data(value)
    }
}

impl<T: Into<u32>> Into<u32> for EnumWrapper<T> {
    fn into(self) -> u32 {
        match self {
            EnumWrapper::Data(data) => data.into(),
            EnumWrapper::Int(val) => val,
        }
    }
}

impl<T: Into<u32>> EnumWrapper<T> {
    pub fn as_u32(self) -> u32 {
        match self {
            EnumWrapper::Data(data) => data.into(),
            EnumWrapper::Int(val) => val,
        }
    }
}

impl<T: Into<u32>, D> Into<Reference<D>> for EnumWrapper<T> {
    fn into(self) -> Reference<D> {
        let id: u32 = self.into();
        id.into()
    }
}
