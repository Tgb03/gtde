use core::fmt;
use std::marker::PhantomData;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};

#[derive(PartialEq, Eq)]
pub struct Reference<T> {
    data: u32,
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

impl<T> Into<u32> for Reference<T> {
    fn into(self) -> u32 {
        self.data
    }
}

impl<T> From<u32> for Reference<T> {
    fn from(value: u32) -> Self {
        Self {
            data: value,
            phantom: PhantomData,
        }
    }
}

impl<T> Serialize for Reference<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.data)
    }
}

impl<'de, T> Deserialize<'de> for Reference<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReferenceVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for ReferenceVisitor<T> {
            type Value = Reference<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a u32 representing a Reference")
            }

            fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Reference {
                    data: value,
                    phantom: PhantomData,
                })
            }

            // Handle other integer types serde might hand us
            // (e.g. from formats like JSON where numbers may come as u64/i64)
            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                let value = u32::try_from(value)
                    .map_err(|_| E::custom(format!("u32 out of range: {}", value)))?;
                self.visit_u32(value)
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                let value = u32::try_from(value)
                    .map_err(|_| E::custom(format!("u32 out of range: {}", value)))?;
                self.visit_u32(value)
            }
        }

        deserializer.deserialize_u32(ReferenceVisitor(PhantomData))
    }
}
