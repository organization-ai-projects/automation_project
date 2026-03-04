use crate::diagnostics::backend_error::BackendError;
use serde::{Deserialize, Serialize};

pub fn encode<T: Serialize>(value: &T) -> Result<String, BackendError> {
    common_json::to_string(value).map_err(|e| BackendError::Serialization(e.to_string()))
}

pub fn decode<T: for<'de> Deserialize<'de>>(data: &str) -> Result<T, BackendError> {
    common_json::from_str(data).map_err(|e| BackendError::Serialization(e.to_string()))
}
