use crate::diagnostics::SpaceEmpireError;
use serde::{Deserialize, Serialize};

pub struct JsonCodec;

impl JsonCodec {
    pub fn encode<T: Serialize>(value: &T) -> Result<String, SpaceEmpireError> {
        serde_json::to_string(value).map_err(|e| SpaceEmpireError::Serialization(e.to_string()))
    }

    pub fn decode<T: for<'de> Deserialize<'de>>(s: &str) -> Result<T, SpaceEmpireError> {
        serde_json::from_str(s).map_err(|e| SpaceEmpireError::Serialization(e.to_string()))
    }
}
