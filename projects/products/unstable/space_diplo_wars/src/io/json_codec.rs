use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::diagnostics::error::SpaceDiploWarsError;

pub struct JsonCodec;

impl JsonCodec {
    /// Encode to canonical JSON with alphabetically sorted object keys.
    pub fn encode<T: Serialize>(value: &T) -> Result<String, SpaceDiploWarsError> {
        let v = serde_json::to_value(value)?;
        let sorted = sort_json_keys(v);
        Ok(serde_json::to_string(&sorted)?)
    }

    /// Decode from JSON.
    pub fn decode<T: DeserializeOwned>(json: &str) -> Result<T, SpaceDiploWarsError> {
        Ok(serde_json::from_str(json)?)
    }
}

/// Recursively sort all JSON object keys alphabetically for canonical output.
pub fn sort_json_keys(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted: serde_json::Map<String, Value> = serde_json::Map::new();
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort();
            for key in keys {
                let v = map[&key].clone();
                sorted.insert(key, sort_json_keys(v));
            }
            Value::Object(sorted)
        }
        Value::Array(arr) => Value::Array(arr.into_iter().map(sort_json_keys).collect()),
        other => other,
    }
}
