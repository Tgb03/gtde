// pub mod schemas;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    Int(i32),
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
