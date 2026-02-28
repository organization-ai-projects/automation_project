use serde::{Deserialize, Serialize};
use crate::diagnostics::BackendError;

pub struct JsonCodec;

impl JsonCodec {
    pub fn encode<T: Serialize>(value: &T) -> Result<String, BackendError> {
        common_json::to_string(value).map_err(|e| BackendError::Codec(e.to_string()))
    }

    pub fn decode<T: for<'de> Deserialize<'de>>(line: &str) -> Result<T, BackendError> {
        common_json::from_json_str(line).map_err(|e| BackendError::Codec(e.to_string()))
    }
}
