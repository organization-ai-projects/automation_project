use serde::{Serialize, de::DeserializeOwned};

pub fn to_json<T: Serialize>(value: &T) -> Result<String, String> {
    common_json::to_string(value).map_err(|e| format!("json encode failed: {e}"))
}

pub fn from_json<T: DeserializeOwned>(json: &str) -> Result<T, String> {
    common_json::from_json_str(json).map_err(|e| format!("json decode failed: {e}"))
}
