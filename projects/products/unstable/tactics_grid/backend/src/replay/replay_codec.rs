use super::replay_file::ReplayFile;
use crate::diagnostics::tactics_grid_error::TacticsGridError;
use std::path::Path;

pub struct ReplayCodec;

impl ReplayCodec {
    pub fn save(replay: &ReplayFile, path: &Path) -> Result<(), TacticsGridError> {
        let json = common_json::to_json_string_pretty(replay)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<ReplayFile, TacticsGridError> {
        let data = std::fs::read_to_string(path)?;
        let replay: ReplayFile = common_json::from_str(&data)?;
        Ok(replay)
    }
}
