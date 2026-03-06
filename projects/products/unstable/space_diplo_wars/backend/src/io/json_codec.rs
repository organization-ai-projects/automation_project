use common_json::Json;
use serde::{Serialize, de::DeserializeOwned};

use crate::diagnostics::error::SpaceDiploWarsError;

pub struct JsonCodec;

impl JsonCodec {
    /// Encode to canonical JSON with alphabetically sorted object keys.
    pub fn encode<T: Serialize>(value: &T) -> Result<String, SpaceDiploWarsError> {
        let v: Json = common_json::to_value(value)?;
        canonical_json_string(&v)
    }

    /// Decode from JSON.
    pub fn decode<T: DeserializeOwned>(json: &str) -> Result<T, SpaceDiploWarsError> {
        Ok(common_json::from_json_str(json)?)
    }
}

/// Recursively sort all JSON object keys alphabetically for canonical output.
pub fn canonical_json_string(value: &Json) -> Result<String, SpaceDiploWarsError> {
    match value {
        Json::Object(map) => {
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort();
            let mut parts: Vec<String> = Vec::with_capacity(keys.len());
            for key in keys {
                let key_json = common_json::to_string(&key)?;
                let value_json = canonical_json_string(&map[&key])?;
                parts.push(format!("{key_json}:{value_json}"));
            }
            Ok(format!("{{{}}}", parts.join(",")))
        }
        Json::Array(arr) => {
            let mut parts: Vec<String> = Vec::with_capacity(arr.len());
            for item in arr {
                parts.push(canonical_json_string(item)?);
            }
            Ok(format!("[{}]", parts.join(",")))
        }
        other => Ok(common_json::to_string(other)?),
    }
}
