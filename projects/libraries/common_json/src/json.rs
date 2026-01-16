use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeMap;
use serde::ser::SerializeSeq;

use crate::JsonVisitor;
use crate::deserialization;
use crate::error;
use crate::serialization;

/// Re-export JSON core types for compatibility.
pub use crate::value::{JsonArray, JsonMap, JsonNumber, JsonObject};

/// Convertit un objet en une chaîne JSON formatée.
pub fn to_json_string_pretty<T: Serialize>(value: &T) -> Result<String, error::JsonError> {
    serialization::to_string_pretty(value)
}

/// Convertit une chaîne JSON en un objet.
pub fn from_json_str<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, error::JsonError> {
    deserialization::from_str(s)
}

/// Valeur JSON générique.
#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Number(JsonNumber),
    String(String),
    Array(JsonArray),
    Object(JsonMap),
}

impl Serialize for Json {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Json::Null => serializer.serialize_unit(),
            Json::Bool(b) => serializer.serialize_bool(*b),
            Json::Number(n) => n.serialize(serializer),
            Json::String(s) => serializer.serialize_str(s),
            Json::Array(arr) => {
                let mut seq = serializer.serialize_seq(Some(arr.len()))?;
                for item in arr {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            Json::Object(map) => {
                let mut m = serializer.serialize_map(Some(map.len()))?;
                for (k, v) in map {
                    m.serialize_entry(k, v)?;
                }
                m.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for Json {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(JsonVisitor)
    }
}
