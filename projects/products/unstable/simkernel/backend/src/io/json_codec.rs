#![allow(dead_code)]
use crate::diagnostics::error::SimError;
use serde::{Deserialize, Serialize};

pub fn encode<T: Serialize>(value: &T) -> Result<String, SimError> {
    serde_json::to_string(value).map_err(|e| SimError::Serialization(e.to_string()))
}

pub fn decode<T: for<'de> Deserialize<'de>>(data: &str) -> Result<T, SimError> {
    serde_json::from_str(data).map_err(|e| SimError::Serialization(e.to_string()))
}
