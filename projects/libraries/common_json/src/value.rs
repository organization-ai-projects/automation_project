// projects/libraries/common_json/src/value.rs
// ============================================================================
// Type Aliases
// ============================================================================

use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, Serializer};

use crate::{JsonError, json::Json, to_json};

/// Key-value map for JSON objects.
pub type JsonMap = std::collections::HashMap<String, Json>;

/// Array of JSON values.
pub type JsonArray = Vec<Json>;

/// JSON object (map String -> Json).
pub type JsonObject = JsonMap;

/// JSON number.
#[derive(Debug, Clone, PartialEq)]
pub struct JsonNumber {
    value: f64,
}

impl Serialize for JsonNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as integer if it has no fractional part
        if self.value.fract() == 0.0 && self.value.abs() < (i64::MAX as f64) {
            serializer.serialize_i64(self.value as i64)
        } else {
            serializer.serialize_f64(self.value)
        }
    }
}

impl<'de> Deserialize<'de> for JsonNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(JsonNumberVisitor)
    }
}

struct JsonNumberVisitor;

impl<'de> Visitor<'de> for JsonNumberVisitor {
    type Value = JsonNumber;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a JSON number")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonNumber { value: v as f64 })
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonNumber { value: v as f64 })
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.is_finite() {
            Ok(JsonNumber { value: v })
        } else {
            Err(de::Error::custom("invalid float value: must be finite"))
        }
    }
}

impl JsonNumber {
    pub fn from_f64(n: f64) -> Option<Self> {
        if n.is_finite() {
            Some(JsonNumber { value: n })
        } else {
            None
        }
    }

    pub fn as_f64(&self) -> f64 {
        self.value
    }

    pub fn is_non_zero(&self) -> bool {
        self.value != 0.0
    }
}

impl From<i64> for JsonNumber {
    fn from(value: i64) -> Self {
        JsonNumber {
            value: value as f64,
        }
    }
}

impl From<u64> for JsonNumber {
    fn from(value: u64) -> Self {
        JsonNumber {
            value: value as f64,
        }
    }
}

pub struct JsonVisitor;

impl<'de> Visitor<'de> for JsonVisitor {
    type Value = Json;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid JSON value")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Json::Null)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Json::Bool(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Json::Number(JsonNumber::from(v)))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Json::Number(JsonNumber::from(v)))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        JsonNumber::from_f64(v)
            .map(Json::Number)
            .ok_or_else(|| de::Error::custom("invalid float value"))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Json::String(v.to_owned()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Json::String(v))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Json::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some(elem) = seq.next_element()? {
            vec.push(elem);
        }
        Ok(Json::Array(vec))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut obj = JsonMap::new();
        while let Some((key, value)) = map.next_entry()? {
            obj.insert(key, value);
        }
        Ok(Json::Object(obj))
    }
}

// ============================================================================
// Constructors
// ============================================================================

/// Creates an empty JSON object `{}`.
#[inline]
pub fn object() -> Json {
    Json::Object(JsonMap::new())
}

/// Creates an empty JSON array `[]`.
#[inline]
pub fn array() -> Json {
    Json::Array(Vec::new())
}

/// Creates a JSON `null` value.
#[inline]
pub fn null() -> Json {
    Json::Null
}

/// Creates a JSON boolean.
#[inline]
pub fn boolean(v: bool) -> Json {
    Json::Bool(v)
}

/// Creates a JSON string.
#[inline]
pub fn string<S: Into<String>>(s: S) -> Json {
    Json::String(s.into())
}

/// Creates a JSON number from an `i64`.
#[inline]
pub fn number_i64(n: i64) -> Json {
    Json::Number(n.into())
}

/// Creates a JSON number from a `u64`.
#[inline]
pub fn number_u64(n: u64) -> Json {
    Json::Number(n.into())
}

/// Creates a JSON number from an `f64`.
#[inline]
pub fn number_f64(n: f64) -> Option<Json> {
    JsonNumber::from_f64(n).map(Json::Number)
}

impl Json {
    pub fn as_str(&self) -> Option<&str> {
        if let Json::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        if let Json::Number(n) = self {
            Some(n.value as i64)
        } else {
            None
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        if let Json::Number(n) = self {
            Some(n.value as u64)
        } else {
            None
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        if let Json::Number(n) = self {
            Some(n.value)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Json::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub fn as_array(&self) -> Option<&JsonArray> {
        if let Json::Array(a) = self {
            Some(a)
        } else {
            None
        }
    }

    pub fn as_object(&self) -> Option<&JsonMap> {
        if let Json::Object(o) = self {
            Some(o)
        } else {
            None
        }
    }

    pub fn is_non_zero(&self) -> bool {
        if let Json::Number(n) = self {
            n.is_non_zero()
        } else {
            false
        }
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Json::Object(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Json::Array(_))
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Json::Null)
    }

    /// Converts a serializable type into `Json`.
    pub fn from_serialize<T: Serialize>(value: &T) -> Result<Self, JsonError> {
        to_json(value)
    }
}

impl From<&str> for Json {
    fn from(value: &str) -> Self {
        Json::String(value.to_string())
    }
}

impl From<String> for Json {
    fn from(value: String) -> Self {
        Json::String(value)
    }
}

impl From<i64> for Json {
    fn from(value: i64) -> Self {
        Json::Number(JsonNumber {
            value: value as f64,
        })
    }
}

impl From<u64> for Json {
    fn from(value: u64) -> Self {
        Json::Number(JsonNumber {
            value: value as f64,
        })
    }
}

impl From<f64> for Json {
    fn from(value: f64) -> Self {
        Json::Number(JsonNumber { value })
    }
}

impl From<bool> for Json {
    fn from(value: bool) -> Self {
        Json::Bool(value)
    }
}

// ============================================================================
// Additional From implementations for common integer types
// ============================================================================

impl From<i8> for Json {
    #[inline]
    fn from(value: i8) -> Self {
        Json::Number(JsonNumber {
            value: value as f64,
        })
    }
}

impl From<i16> for Json {
    #[inline]
    fn from(value: i16) -> Self {
        Json::Number(JsonNumber {
            value: value as f64,
        })
    }
}

impl From<i32> for Json {
    #[inline]
    fn from(value: i32) -> Self {
        Json::Number(JsonNumber {
            value: value as f64,
        })
    }
}

impl From<u8> for Json {
    #[inline]
    fn from(value: u8) -> Self {
        Json::Number(JsonNumber {
            value: value as f64,
        })
    }
}

impl From<u16> for Json {
    #[inline]
    fn from(value: u16) -> Self {
        Json::Number(JsonNumber {
            value: value as f64,
        })
    }
}

impl From<u32> for Json {
    #[inline]
    fn from(value: u32) -> Self {
        Json::Number(JsonNumber {
            value: value as f64,
        })
    }
}

impl From<f32> for Json {
    #[inline]
    fn from(value: f32) -> Self {
        Json::Number(JsonNumber {
            value: value as f64,
        })
    }
}

impl From<isize> for Json {
    #[inline]
    fn from(value: isize) -> Self {
        Json::Number(JsonNumber {
            value: value as f64,
        })
    }
}

impl From<usize> for Json {
    #[inline]
    fn from(value: usize) -> Self {
        Json::Number(JsonNumber {
            value: value as f64,
        })
    }
}

// Option support: None -> Null, Some(v) -> v.into()
impl<T: Into<Json>> From<Option<T>> for Json {
    #[inline]
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => v.into(),
            None => Json::Null,
        }
    }
}

// Vec support
impl<T: Into<Json>> From<Vec<T>> for Json {
    #[inline]
    fn from(value: Vec<T>) -> Self {
        Json::Array(value.into_iter().map(Into::into).collect())
    }
}

// &[T] slice support (requires Clone)
impl<T: Clone + Into<Json>> From<&[T]> for Json {
    #[inline]
    fn from(value: &[T]) -> Self {
        Json::Array(value.iter().cloned().map(Into::into).collect())
    }
}

// Cow<str> support
impl From<std::borrow::Cow<'_, str>> for Json {
    #[inline]
    fn from(value: std::borrow::Cow<'_, str>) -> Self {
        Json::String(value.into_owned())
    }
}

// &String support
impl From<&String> for Json {
    #[inline]
    fn from(value: &String) -> Self {
        Json::String(value.clone())
    }
}

// char support
impl From<char> for Json {
    #[inline]
    fn from(value: char) -> Self {
        Json::String(value.to_string())
    }
}

// () unit type -> Null
impl From<()> for Json {
    #[inline]
    fn from(_: ()) -> Self {
        Json::Null
    }
}
