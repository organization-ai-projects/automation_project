// projects/libraries/protocol/src/json.rs
use serde::{Serialize, de::DeserializeOwned};

pub type Json = serde_json::Value;

// Re-export the json! macro
pub use serde_json::json;

pub fn to_json<T: Serialize>(value: &T) -> Result<Json, serde_json::Error> {
    serde_json::to_value(value)
}

pub fn to_json_string<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(value)
}

pub fn to_json_string_pretty<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(value)
}

pub fn from_json<T: DeserializeOwned>(value: &Json) -> Result<T, serde_json::Error> {
    serde_json::from_value(value.clone())
}

pub fn from_json_owned<T: DeserializeOwned>(value: Json) -> Result<T, serde_json::Error> {
    serde_json::from_value(value)
}

pub fn from_json_str<T: DeserializeOwned>(s: &str) -> Result<T, serde_json::Error> {
    serde_json::from_str(s)
}
