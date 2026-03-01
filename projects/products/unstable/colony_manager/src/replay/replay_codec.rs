use crate::replay::replay_file::ReplayFile;
use crate::diagnostics::error::ColonyManagerError;
use std::path::Path;

pub struct ReplayCodec;

impl ReplayCodec {
    pub fn save(replay: &ReplayFile, path: &Path) -> Result<(), ColonyManagerError> {
        let json = serde_json::to_string_pretty(replay)?;
        std::fs::write(path, json)?;
        Ok(())
    }
    pub fn load(path: &Path) -> Result<ReplayFile, ColonyManagerError> {
        let data = std::fs::read_to_string(path)?;
        let replay = serde_json::from_str(&data)?;
        Ok(replay)
    }
}
