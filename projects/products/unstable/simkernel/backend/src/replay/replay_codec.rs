#![allow(dead_code)]
use crate::diagnostics::error::SimError;
use crate::replay::replay_file::ReplayFile;

pub struct ReplayCodec;

impl ReplayCodec {
    pub fn encode(file: &ReplayFile) -> Result<String, SimError> {
        serde_json::to_string(file).map_err(|e| SimError::Serialization(e.to_string()))
    }

    pub fn decode(data: &str) -> Result<ReplayFile, SimError> {
        serde_json::from_str(data).map_err(|e| SimError::Serialization(e.to_string()))
    }
}
