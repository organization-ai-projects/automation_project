use crate::diagnostics::error::ColonyManagerError;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub struct JsonCodec;

impl JsonCodec {
    pub fn save<T: Serialize>(value: &T, path: &Path) -> Result<(), ColonyManagerError> {
        let json = serde_json::to_string_pretty(value)?;
        std::fs::write(path, json)?;
        Ok(())
    }
    pub fn load<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, ColonyManagerError> {
        let data = std::fs::read_to_string(path)?;
        let value = serde_json::from_str(&data)?;
        Ok(value)
    }
}
