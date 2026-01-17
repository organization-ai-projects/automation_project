// projects/libraries/common_json/src/deserialization/json_variant_access.rs
use crate::{Json, json_error::JsonError};
use serde::de::{DeserializeSeed, VariantAccess, Visitor};

use super::helpers::type_error;
use super::json_deserializer::JsonDeserializer;
use super::json_map_access::JsonMapAccess;
use super::json_seq_access::JsonSeqAccess;
use crate::json_error::JsonErrorCode;

pub(crate) struct JsonVariantAccess<'de> {
    pub(crate) value: Option<&'de Json>,
}

impl<'de> VariantAccess<'de> for JsonVariantAccess<'de> {
    type Error = JsonError;

    fn unit_variant(self) -> Result<(), JsonError> {
        match self.value {
            None => Ok(()),
            Some(Json::Null) => Ok(()),
            Some(other) => Err(type_error("unit", other)),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, JsonError>
    where
        T: DeserializeSeed<'de>,
    {
        let value = self
            .value
            .ok_or_else(|| JsonError::new(JsonErrorCode::MissingEnumValueError))?;
        seed.deserialize(JsonDeserializer::new(value))
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Json::Array(values)) => visitor.visit_seq(JsonSeqAccess::new(values.iter())),
            Some(other) => Err(type_error("array", other)),
            None => Err(JsonError::new(JsonErrorCode::MissingEnumValueError)),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Json::Object(map)) => visitor.visit_map(JsonMapAccess::new(map.iter())),
            Some(other) => Err(type_error("object", other)),
            None => Err(JsonError::new(JsonErrorCode::MissingEnumValueError)),
        }
    }
}
