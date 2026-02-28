// projects/products/unstable/simulation_compiler/backend/src/io/json_codec.rs
use crate::diagnostics::error::CompilerError;

pub fn encode<T: serde::Serialize>(value: &T) -> Result<String, CompilerError> {
    common_json::to_string(value).map_err(|e| CompilerError::Io(e.to_string()))
}

pub fn decode<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, CompilerError> {
    common_json::from_json_str(s).map_err(|e| CompilerError::Io(e.to_string()))
}
