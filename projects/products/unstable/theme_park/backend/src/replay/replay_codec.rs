#![allow(dead_code)]
use crate::diagnostics::error::SimError;
use crate::replay::replay_file::ReplayFile;

/// Load and save replay files from/to JSON.
pub struct ReplayCodec;

impl ReplayCodec {
    pub fn save(replay: &ReplayFile, path: &str) -> Result<(), SimError> {
        let json =
            serde_json::to_string_pretty(replay).map_err(|e| SimError::Serialization(e.to_string()))?;
        std::fs::write(path, json).map_err(|e| SimError::Io(e.to_string()))
    }

    pub fn load(path: &str) -> Result<ReplayFile, SimError> {
        let data = std::fs::read_to_string(path).map_err(|e| SimError::Io(e.to_string()))?;
        serde_json::from_str(&data).map_err(|e| SimError::Serialization(e.to_string()))
    }
}
