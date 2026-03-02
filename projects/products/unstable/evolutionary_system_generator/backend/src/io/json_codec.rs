use serde::{Deserialize, Serialize};

pub fn encode<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(value)
}

pub fn decode<T: for<'de> Deserialize<'de>>(s: &str) -> Result<T, serde_json::Error> {
    serde_json::from_str(s)
}
