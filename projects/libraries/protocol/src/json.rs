// projects/libraries/protocol/src/json.rs
use serde::{Deserialize, Serialize};

pub type Json = serde_json::Value;

pub fn to_json<T: Serialize>(value: &T) -> Json {
    serde_json::to_value(value).expect("json serialization failed")
}

pub fn from_json<T: for<'de> Deserialize<'de>>(value: Json) -> Result<T, serde_json::Error> {
    serde_json::from_value(value)
}
