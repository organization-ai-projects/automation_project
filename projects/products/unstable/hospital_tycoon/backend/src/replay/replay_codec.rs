// projects/products/unstable/hospital_tycoon/backend/src/replay/replay_codec.rs
use crate::diagnostics::error::AppError;
use crate::replay::replay_file::ReplayFile;
use std::fs;

pub struct ReplayCodec;

impl ReplayCodec {
    pub fn save(replay: &ReplayFile, path: &str) -> Result<(), AppError> {
        let json = serde_json::to_string_pretty(replay)
            .map_err(|e| AppError::Io(e.to_string()))?;
        fs::write(path, json).map_err(|e| AppError::Io(e.to_string()))
    }

    pub fn load(path: &str) -> Result<ReplayFile, AppError> {
        let data = fs::read_to_string(path).map_err(|e| AppError::Io(e.to_string()))?;
        serde_json::from_str(&data).map_err(|e| AppError::Replay(e.to_string()))
    }
}
