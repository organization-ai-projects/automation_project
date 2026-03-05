use crate::diagnostics::colony_manager_error::ColonyManagerError;
use crate::replay::replay_file::ReplayFile;
use std::path::Path;

pub struct ReplayCodec;

impl ReplayCodec {
    pub fn save(replay: &ReplayFile, path: &Path) -> Result<(), ColonyManagerError> {
        let json = common_json::to_json_string_pretty(replay)
            .map_err(|e| ColonyManagerError::Json(e.to_string()))?;
        std::fs::write(path, json)?;
        Ok(())
    }
    pub fn load(path: &Path) -> Result<ReplayFile, ColonyManagerError> {
        let data = std::fs::read_to_string(path)?;
        let replay =
            common_json::from_str(&data).map_err(|e| ColonyManagerError::Json(e.to_string()))?;
        Ok(replay)
    }
}
