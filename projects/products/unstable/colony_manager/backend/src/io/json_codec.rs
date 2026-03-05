use crate::diagnostics::colony_manager_error::ColonyManagerError;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub struct JsonCodec;

impl JsonCodec {
    pub fn save<T: Serialize>(value: &T, path: &Path) -> Result<(), ColonyManagerError> {
        let json = common_json::to_json_string_pretty(value)
            .map_err(|e| ColonyManagerError::Json(e.to_string()))?;
        std::fs::write(path, json)?;
        Ok(())
    }
    pub fn load<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, ColonyManagerError> {
        let data = std::fs::read_to_string(path)?;
        let value =
            common_json::from_str(&data).map_err(|e| ColonyManagerError::Json(e.to_string()))?;
        Ok(value)
    }
}
