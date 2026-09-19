use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
