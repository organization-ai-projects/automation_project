// projects/products/unstable/protocol_builder/backend/src/io/json_codec.rs
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub fn encode<T: Serialize>(value: &T) -> Result<String> {
    let s = common_json::to_string(value)?;
    Ok(s)
}

pub fn decode<T: for<'de> Deserialize<'de>>(s: &str) -> Result<T> {
    let value = common_json::from_json_str(s)?;
    Ok(value)
}
