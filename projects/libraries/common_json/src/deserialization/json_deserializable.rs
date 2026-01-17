// projects/libraries/common_json/src/deserialization/json_deserializable.rs
use crate::json_error::JsonResult;
use crate::{Json, parser};
use serde::de::DeserializeOwned;
use std::io::Read;

use super::json_deserializer::JsonDeserializer;

/// Trait for types that can be deserialized from JSON.
/// See the full documentation in docs/deserialize.md.
pub trait JsonDeserializable: Sized {
    fn from_json(value: &Json) -> JsonResult<Self>;
    fn from_json_owned(value: Json) -> JsonResult<Self>;
    fn from_json_str(s: &str) -> JsonResult<Self>;
    fn from_json_bytes(bytes: &[u8]) -> JsonResult<Self>;
    fn from_json_reader<R: Read>(reader: R) -> JsonResult<Self>;
}

impl<T> JsonDeserializable for T
where
    T: DeserializeOwned,
{
    fn from_json(value: &Json) -> JsonResult<Self> {
        T::deserialize(JsonDeserializer::new(value))
    }

    fn from_json_owned(value: Json) -> JsonResult<Self> {
        T::deserialize(JsonDeserializer::new(&value))
    }

    fn from_json_str(s: &str) -> JsonResult<Self> {
        let value = parse(s)?;
        Self::from_json(&value)
    }

    fn from_json_bytes(bytes: &[u8]) -> JsonResult<Self> {
        let value = parse_bytes(bytes)?;
        Self::from_json(&value)
    }

    fn from_json_reader<R: Read>(reader: R) -> JsonResult<Self> {
        let value = parse_reader(reader)?;
        Self::from_json(&value)
    }
}

// Standalone functions for parsing and deserialization.
// See details and examples in docs/deserialize.md.
#[inline]
pub fn parse(s: &str) -> JsonResult<Json> {
    parser::parse_str(s)
}

#[inline]
pub fn parse_bytes(bytes: &[u8]) -> JsonResult<Json> {
    parser::parse_bytes(bytes)
}

#[inline]
pub fn parse_reader<R: Read>(reader: R) -> JsonResult<Json> {
    parser::parse_reader(reader)
}

#[inline]
pub fn from_json<T: JsonDeserializable>(value: &Json) -> JsonResult<T> {
    T::from_json(value)
}

#[inline]
pub fn from_json_owned<T: JsonDeserializable>(value: Json) -> JsonResult<T> {
    T::from_json_owned(value)
}

#[inline]
pub fn from_str<T: JsonDeserializable>(s: &str) -> JsonResult<T> {
    let json_value = parse(s)?;
    T::from_json(&json_value)
}

#[inline]
pub fn from_bytes<T: JsonDeserializable>(bytes: &[u8]) -> JsonResult<T> {
    let json_value = parse_bytes(bytes)?;
    T::from_json(&json_value)
}

#[inline]
pub fn from_reader<T: JsonDeserializable, R: Read>(reader: R) -> JsonResult<T> {
    let json_value = parse_reader(reader)?;
    T::from_json(&json_value)
}

// Legacy alias for backward compatibility.
#[inline]
pub fn from_value<T: JsonDeserializable>(value: Json) -> JsonResult<T> {
    from_json_owned(value)
}

#[inline]
pub fn from_json_str<T: JsonDeserializable>(s: &str) -> JsonResult<T> {
    from_str(s)
}

#[inline]
pub fn from_slice<T: JsonDeserializable>(bytes: &[u8]) -> JsonResult<T> {
    from_bytes(bytes)
}
