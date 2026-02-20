// projects/libraries/common_json/src/json.rs
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeMap;
use serde::ser::SerializeSeq;
use std::collections::HashMap;

use crate::JsonVisitor;
use crate::deserialization;
use crate::json_error;
use crate::serialization;

/// Re-export JSON core types for compatibility.
pub use crate::value::{JsonArray, JsonMap, JsonNumber, JsonObject};

/// Converts an object into a formatted JSON string.
pub fn to_json_string_pretty<T: Serialize>(value: &T) -> Result<String, json_error::JsonError> {
    serialization::to_string_pretty(value)
}

/// Converts a JSON string into an object.
pub fn from_json_str<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, json_error::JsonError> {
    deserialization::from_str(s)
}

/// Generic JSON value.
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

impl Json {
    /// Extracts the structure of a JSON object, preserving keys and values.
    pub fn extract_structure(&self) -> HashMap<String, String> {
        let mut structure = HashMap::new();
        self.extract_recursive("root", &mut structure);
        structure
    }

    fn extract_recursive(&self, path: &str, structure: &mut HashMap<String, String>) {
        match self {
            Json::Object(map) => {
                for (key, value) in map {
                    let new_path = format!("{}/{}", path, key);
                    value.extract_recursive(&new_path, structure);
                }
            }
            Json::Array(array) => {
                for (index, value) in array.iter().enumerate() {
                    let new_path = format!("{}/[{}]", path, index);
                    value.extract_recursive(&new_path, structure);
                }
            }
            _ => {
                structure.insert(path.to_string(), format!("{:?}", self));
            }
        }
    }
}
