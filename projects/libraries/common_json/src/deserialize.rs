use crate::error::{JsonError, JsonResult};
use crate::parser;
use crate::value::Json;
use serde::de::{
    self, DeserializeOwned, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};
use std::io::Read;

/// Trait pour les types désérialisables depuis JSON.
/// Voir la documentation complète dans docs/deserialize.md.
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

// Fonctions standalone pour le parsing et la désérialisation.
// Voir les détails et exemples dans docs/deserialize.md.
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

// Alias legacy pour compatibilité ascendante.
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

struct JsonDeserializer<'de> {
    input: &'de Json,
}

impl<'de> JsonDeserializer<'de> {
    fn new(input: &'de Json) -> Self {
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
        let value = i8::try_from(value)
            .map_err(|_| JsonError::custom("integer out of range"))?;
        visitor.visit_i8(value)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        let value = to_i64(self.input)?;
        let value = i16::try_from(value)
            .map_err(|_| JsonError::custom("integer out of range"))?;
        visitor.visit_i16(value)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        let value = to_i64(self.input)?;
        let value = i32::try_from(value)
            .map_err(|_| JsonError::custom("integer out of range"))?;
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
        let value = u8::try_from(value)
            .map_err(|_| JsonError::custom("integer out of range"))?;
        visitor.visit_u8(value)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        let value = to_u64(self.input)?;
        let value = u16::try_from(value)
            .map_err(|_| JsonError::custom("integer out of range"))?;
        visitor.visit_u16(value)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        let value = to_u64(self.input)?;
        let value = u32::try_from(value)
            .map_err(|_| JsonError::custom("integer out of range"))?;
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
                    Err(JsonError::custom("expected a single character"))
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

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, JsonError>
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
                let (variant, value) = map.iter().next().unwrap();
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

struct JsonSeqAccess<'de, I> {
    iter: I,
    marker: std::marker::PhantomData<&'de Json>,
}

impl<'de, I> JsonSeqAccess<'de, I>
where
    I: Iterator<Item = &'de Json>,
{
    fn new(iter: I) -> Self {
        Self {
            iter,
            marker: std::marker::PhantomData,
        }
    }
}

impl<'de, I> SeqAccess<'de> for JsonSeqAccess<'de, I>
where
    I: Iterator<Item = &'de Json>,
{
    type Error = JsonError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, JsonError>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(value) = self.iter.next() {
            seed.deserialize(JsonDeserializer::new(value)).map(Some)
        } else {
            Ok(None)
        }
    }
}

struct JsonMapAccess<'de, I> {
    iter: I,
    value: Option<&'de Json>,
}

impl<'de, I> JsonMapAccess<'de, I>
where
    I: Iterator<Item = (&'de String, &'de Json)>,
{
    fn new(iter: I) -> Self {
        Self { iter, value: None }
    }
}

impl<'de, I> MapAccess<'de> for JsonMapAccess<'de, I>
where
    I: Iterator<Item = (&'de String, &'de Json)>,
{
    type Error = JsonError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, JsonError>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some((key, value)) = self.iter.next() {
            self.value = Some(value);
            let key = key.as_str().into_deserializer();
            seed.deserialize(key).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, JsonError>
    where
        V: DeserializeSeed<'de>,
    {
        let value = self
            .value
            .take()
            .ok_or_else(|| JsonError::custom("value is missing"))?;
        seed.deserialize(JsonDeserializer::new(value))
    }
}

struct JsonEnumAccess<'de> {
    variant: &'de str,
    value: Option<&'de Json>,
}

impl<'de> JsonEnumAccess<'de> {
    fn new(variant: &'de str, value: Option<&'de Json>) -> Self {
        Self { variant, value }
    }
}

impl<'de> EnumAccess<'de> for JsonEnumAccess<'de> {
    type Error = JsonError;
    type Variant = JsonVariantAccess<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), JsonError>
    where
        V: DeserializeSeed<'de>,
    {
        let value = seed.deserialize(self.variant.into_deserializer())?;
        Ok((value, JsonVariantAccess { value: self.value }))
    }
}

struct JsonVariantAccess<'de> {
    value: Option<&'de Json>,
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
            .ok_or_else(|| JsonError::custom("missing enum value"))?;
        seed.deserialize(JsonDeserializer::new(value))
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Json::Array(values)) => visitor.visit_seq(JsonSeqAccess::new(values.iter())),
            Some(other) => Err(type_error("array", other)),
            None => Err(JsonError::custom("missing enum value")),
        }
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, JsonError>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Json::Object(map)) => visitor.visit_map(JsonMapAccess::new(map.iter())),
            Some(other) => Err(type_error("object", other)),
            None => Err(JsonError::custom("missing enum value")),
        }
    }
}

fn json_type_name(value: &Json) -> &'static str {
    match value {
        Json::Null => "null",
        Json::Bool(_) => "bool",
        Json::Number(_) => "number",
        Json::String(_) => "string",
        Json::Array(_) => "array",
        Json::Object(_) => "object",
    }
}

fn type_error(expected: &'static str, found: &Json) -> JsonError {
    JsonError::type_mismatch(expected, json_type_name(found))
}

fn to_i64(value: &Json) -> JsonResult<i64> {
    match value {
        Json::Number(number) => {
            let v = number.as_f64();
            if v.fract() == 0.0 && v >= i64::MIN as f64 && v <= i64::MAX as f64 {
                Ok(v as i64)
            } else {
                Err(JsonError::custom("invalid integer"))
            }
        }
        other => Err(type_error("number", other)),
    }
}

fn to_u64(value: &Json) -> JsonResult<u64> {
    match value {
        Json::Number(number) => {
            let v = number.as_f64();
            if v.fract() == 0.0 && v >= 0.0 && v <= u64::MAX as f64 {
                Ok(v as u64)
            } else {
                Err(JsonError::custom("invalid integer"))
            }
        }
        other => Err(type_error("number", other)),
    }
}

fn to_f64(value: &Json) -> JsonResult<f64> {
    match value {
        Json::Number(number) => Ok(number.as_f64()),
        other => Err(type_error("number", other)),
    }
}

fn to_bytes(value: &Json) -> JsonResult<Vec<u8>> {
    match value {
        Json::Array(values) => {
            let mut buffer = Vec::with_capacity(values.len());
            for item in values {
                let number = match item {
                    Json::Number(number) => number.as_f64(),
                    other => return Err(type_error("number", other)),
                };
                if number.fract() != 0.0 || number < 0.0 || number > u8::MAX as f64 {
                    return Err(JsonError::custom("invalid byte value"));
                }
                buffer.push(number as u8);
            }
            Ok(buffer)
        }
        other => Err(type_error("array", other)),
    }
}
