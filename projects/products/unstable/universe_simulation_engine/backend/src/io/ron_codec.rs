use crate::diagnostics::engine_error::EngineError;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::path::Path;

pub fn save_ron<T: Serialize>(value: &T, path: &Path) -> Result<(), EngineError> {
    common_ron::write_ron(path, value).map_err(|e| EngineError::RonCodec(e.to_string()))
}

pub fn load_ron<T: DeserializeOwned>(path: &Path) -> Result<T, EngineError> {
    common_ron::read_ron(path).map_err(|e| EngineError::RonCodec(e.to_string()))
}
