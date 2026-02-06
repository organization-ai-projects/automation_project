// projects/libraries/common_json/src/serialization/json_serializer.rs
use crate::Json;
use crate::json_error::JsonError;
use crate::json_error_code::JsonErrorCode;
use crate::value::{JsonMap, JsonNumber};
use serde::ser::{self, Serialize};

use super::json_map_serializer::JsonMapSerializer;
use super::json_seq_serializer::JsonSeqSerializer;
use super::json_struct_variant_serializer::JsonStructVariantSerializer;
use super::json_tuple_variant_serializer::JsonTupleVariantSerializer;

pub(crate) struct JsonSerializer;

impl ser::Serializer for JsonSerializer {
    type Ok = Json;
    type Error = JsonError;

    type SerializeSeq = JsonSeqSerializer;
    type SerializeTuple = JsonSeqSerializer;
    type SerializeTupleStruct = JsonSeqSerializer;
    type SerializeTupleVariant = JsonTupleVariantSerializer;
    type SerializeMap = JsonMapSerializer;
    type SerializeStruct = JsonMapSerializer;
    type SerializeStructVariant = JsonStructVariantSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Json::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Json::Number(JsonNumber::from(v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Json::Number(JsonNumber::from(v)))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        if !v.is_finite() {
            return Err(JsonError::new(JsonErrorCode::Serialize));
        }
        let number =
            JsonNumber::from_f64(v).ok_or_else(|| JsonError::new(JsonErrorCode::Serialize))?;
        Ok(Json::Number(number))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Json::String(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Json::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let values = v
            .iter()
            .map(|b| Json::Number(JsonNumber::from(*b as u64)))
            .collect();
        Ok(Json::Array(values))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Json::Null)
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Json::Null)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Json::Null)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Json::String(variant.to_string()))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        let mut map = JsonMap::new();
        map.insert(variant.to_string(), value.serialize(self)?);
        Ok(Json::Object(map))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(JsonSeqSerializer {
            elements: Vec::with_capacity(len.unwrap_or(0)),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(JsonTupleVariantSerializer {
            name: variant.to_string(),
            elements: Vec::with_capacity(len),
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(JsonMapSerializer::with_capacity(len.unwrap_or(0)))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(JsonStructVariantSerializer {
            name: variant.to_string(),
            map: JsonMap::with_capacity(len),
        })
    }
}
