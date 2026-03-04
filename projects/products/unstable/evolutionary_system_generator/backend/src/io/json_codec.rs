// projects/products/unstable/evolutionary_system_generator/backend/src/io/json_codec.rs
use serde::{Serialize, de::DeserializeOwned};

pub fn encode<T: Serialize>(value: &T) -> Result<String, common_json::JsonError> {
    common_json::to_string(value)
}

pub fn decode<T: DeserializeOwned>(s: &str) -> Result<T, common_json::JsonError> {
    common_json::from_json_str(s)
}
