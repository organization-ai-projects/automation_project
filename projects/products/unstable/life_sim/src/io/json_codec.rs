use crate::diagnostics::LifeSimError;
use std::fs;
use std::path::Path;

pub struct JsonCodec;

impl JsonCodec {
    pub fn read_file<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, LifeSimError> {
        let content = fs::read_to_string(path).map_err(|e| LifeSimError::Io(e.to_string()))?;
        serde_json::from_str(&content).map_err(|e| LifeSimError::Serialization(e.to_string()))
    }

    pub fn write_file<T: serde::Serialize>(path: &Path, value: &T) -> Result<(), LifeSimError> {
        let content = serde_json::to_string_pretty(value)
            .map_err(|e| LifeSimError::Serialization(e.to_string()))?;
        fs::write(path, content).map_err(|e| LifeSimError::Io(e.to_string()))
    }
}
