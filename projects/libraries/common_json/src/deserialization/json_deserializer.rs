// projects/libraries/common_json/src/deserialization/json_deserializer.rs
use crate::{Json, json_error::JsonError};
use serde::de::{self, Visitor};

use super::helpers::{to_bytes, to_f64, to_i64, to_u64, type_error};
use super::json_enum_access::JsonEnumAccess;
use super::json_map_access::JsonMapAccess;
use super::json_seq_access::JsonSeqAccess;
use crate::json_error::JsonErrorCode;

pub(crate) struct JsonDeserializer<'de> {
    input: &'de Json,
}

impl<'de> JsonDeserializer<'de> {
    pub(crate) fn new(input: &'de Json) -> Self {
        Self { input }
    }
}

impl<'de> de::Deserializer<'de> for JsonDeserializer<'de> {
    type Error = JsonError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Json::Null => visitor.visit_unit(),
            Json::Bool(value) => visitor.visit_bool(*value),
            Json::Number(number) => visitor.visit_f64(number.as_f64()),
            Json::String(value) => visitor.visit_str(value),
            Json::Array(values) => visitor.visit_seq(JsonSeqAccess::new(values.iter())),
            Json::Object(map) => visitor.visit_map(JsonMapAccess::new(map.iter())),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Json::Bool(value) => visitor.visit_bool(*value),
            other => Err(type_error("bool", other)),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        let value = to_i64(self.input)?;
        let value =
            i8::try_from(value).map_err(|_| JsonError::new(JsonErrorCode::InvalidInteger))?;
        visitor.visit_i8(value)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        let value = to_i64(self.input)?;
        let value =
            i16::try_from(value).map_err(|_| JsonError::new(JsonErrorCode::InvalidInteger))?;
        visitor.visit_i16(value)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        let value = to_i64(self.input)?;
        let value =
            i32::try_from(value).map_err(|_| JsonError::new(JsonErrorCode::InvalidInteger))?;
        visitor.visit_i32(value)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        let value = to_i64(self.input)?;
        visitor.visit_i64(value)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        let value = to_u64(self.input)?;
        let value =
            u8::try_from(value).map_err(|_| JsonError::new(JsonErrorCode::InvalidInteger))?;
        visitor.visit_u8(value)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        let value = to_u64(self.input)?;
        let value =
            u16::try_from(value).map_err(|_| JsonError::new(JsonErrorCode::InvalidInteger))?;
        visitor.visit_u16(value)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        let value = to_u64(self.input)?;
        let value =
            u32::try_from(value).map_err(|_| JsonError::new(JsonErrorCode::InvalidInteger))?;
        visitor.visit_u32(value)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        let value = to_u64(self.input)?;
        visitor.visit_u64(value)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        let value = to_f64(self.input)?;
        visitor.visit_f32(value as f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        let value = to_f64(self.input)?;
        visitor.visit_f64(value)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Json::String(value) => {
                let mut chars = value.chars();
                if let (Some(ch), None) = (chars.next(), chars.next()) {
                    visitor.visit_char(ch)
                } else {
                    Err(JsonError::new(JsonErrorCode::ExpectedSingleCharacter))
                }
            }
            other => Err(type_error("string", other)),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Json::String(value) => visitor.visit_str(value),
            other => Err(type_error("string", other)),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Json::String(value) => visitor.visit_string(value.clone()),
            other => Err(type_error("string", other)),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        let bytes = to_bytes(self.input)?;
        visitor.visit_bytes(&bytes)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        let bytes = to_bytes(self.input)?;
        visitor.visit_byte_buf(bytes)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Json::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Json::Null => visitor.visit_unit(),
            other => Err(type_error("null", other)),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Json::Array(values) => visitor.visit_seq(JsonSeqAccess::new(values.iter())),
            other => Err(type_error("array", other)),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Json::Object(map) => visitor.visit_map(JsonMapAccess::new(map.iter())),
            other => Err(type_error("object", other)),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Json::String(variant) => visitor.visit_enum(JsonEnumAccess::new(variant, None)),
            Json::Object(map) if map.len() == 1 => {
                let (variant, value) = map.iter().next().expect("enum object has one entry");
                visitor.visit_enum(JsonEnumAccess::new(variant, Some(value)))
            }
            other => Err(type_error("enum", other)),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}
