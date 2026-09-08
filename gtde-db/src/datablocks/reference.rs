use std::marker::PhantomData;

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(PartialEq, Eq, Debug)]
pub struct Reference<T> {
    data: i64,
    phantom: PhantomData<T>,
}

impl<T> Default for Reference<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            phantom: Default::default(),
        }
    }
}

impl<T> Clone for Reference<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            phantom: PhantomData,
        }
    }
}

impl<T> Copy for Reference<T> {}

impl<T> Into<i64> for Reference<T> {
    fn into(self) -> i64 {
        self.data
    }
}

impl<T> Reference<T> {
    pub fn as_i64(self) -> i64 {
        self.data
    }
}

impl<T> From<i64> for Reference<T> {
    fn from(value: i64) -> Self {
        Self {
            data: value,
            phantom: PhantomData,
        }
    }
}

impl<T> From<u32> for Reference<T> {
    fn from(value: u32) -> Self {
        Self {
            data: value as i64,
            phantom: PhantomData,
        }
    }
}

impl<T> Serialize for Reference<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.data)
    }
}

impl<'de, T> Deserialize<'de> for Reference<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = i64::deserialize(deserializer)?;
        Ok(Self::from(data))
    }
}

impl<T> JsonSchema for Reference<T> {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Reference".into()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        i64::json_schema(generator)
    }
}
