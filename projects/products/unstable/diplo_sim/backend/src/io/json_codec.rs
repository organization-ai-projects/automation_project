use crate::diagnostics::error::DiploSimError;

/// Encode a value to JSON string.
pub fn encode<T: serde::Serialize>(value: &T) -> Result<String, DiploSimError> {
    common_json::to_json_string(value)
        .map_err(|e| DiploSimError::Internal(format!("JSON encode error: {e}")))
}

/// Decode a value from JSON string.
pub fn decode<T: serde::de::DeserializeOwned>(json: &str) -> Result<T, DiploSimError> {
    common_json::from_str(json)
        .map_err(|e| DiploSimError::Internal(format!("JSON decode error: {e}")))
}
