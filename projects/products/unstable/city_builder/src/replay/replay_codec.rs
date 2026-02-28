use super::replay_file::ReplayFile;
use crate::diagnostics::error::CityBuilderError;
use std::path::Path;

pub struct ReplayCodec;

impl ReplayCodec {
    pub fn load(path: &Path) -> Result<ReplayFile, CityBuilderError> {
        let s = std::fs::read_to_string(path)
            .map_err(|e| CityBuilderError::Io(e.to_string()))?;
        serde_json::from_str(&s)
            .map_err(|e| CityBuilderError::InvalidScenario(e.to_string()))
    }
}
