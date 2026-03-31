use crate::diagnostics::RogueliteArenaError;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub(crate) struct JsonCodec;

impl JsonCodec {
    pub(crate) fn save<T: Serialize>(value: &T, path: &Path) -> Result<(), RogueliteArenaError> {
        let json = common_json::to_json_string_pretty(value)
            .map_err(|e| RogueliteArenaError::Json(e.to_string()))?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub(crate) fn load<T: for<'de> Deserialize<'de>>(
        path: &Path,
    ) -> Result<T, RogueliteArenaError> {
        let data = std::fs::read_to_string(path)?;
        let value =
            common_json::from_str(&data).map_err(|e| RogueliteArenaError::Json(e.to_string()))?;
        Ok(value)
    }
}
