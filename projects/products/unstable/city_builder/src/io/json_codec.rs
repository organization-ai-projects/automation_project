use crate::diagnostics::error::CityBuilderError;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub struct JsonCodec;

impl JsonCodec {
    pub fn write<T: Serialize>(value: &T, path: &Path) -> Result<(), CityBuilderError> {
        let s =
            serde_json::to_string_pretty(value).map_err(|e| CityBuilderError::Io(e.to_string()))?;
        std::fs::write(path, s).map_err(|e| CityBuilderError::Io(e.to_string()))
    }

    pub fn read<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, CityBuilderError> {
        let s = std::fs::read_to_string(path).map_err(|e| CityBuilderError::Io(e.to_string()))?;
        serde_json::from_str(&s).map_err(|e| CityBuilderError::Io(e.to_string()))
    }
}
