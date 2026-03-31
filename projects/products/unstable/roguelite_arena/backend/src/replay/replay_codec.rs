use crate::diagnostics::RogueliteArenaError;
use crate::replay::ReplayFile;
use std::path::Path;

pub(crate) struct ReplayCodec;

impl ReplayCodec {
    pub(crate) fn save(replay: &ReplayFile, path: &Path) -> Result<(), RogueliteArenaError> {
        let json = common_json::to_json_string_pretty(replay)
            .map_err(|e| RogueliteArenaError::Json(e.to_string()))?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub(crate) fn load(path: &Path) -> Result<ReplayFile, RogueliteArenaError> {
        let data = std::fs::read_to_string(path)?;
        let replay =
            common_json::from_str(&data).map_err(|e| RogueliteArenaError::Json(e.to_string()))?;
        Ok(replay)
    }
}
