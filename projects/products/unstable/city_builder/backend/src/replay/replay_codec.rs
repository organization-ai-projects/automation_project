use super::replay_file::ReplayFile;
use crate::diagnostics::city_builder_error::CityBuilderError;
use std::path::Path;

pub struct ReplayCodec;

impl ReplayCodec {
    pub fn save(path: &Path, replay: &ReplayFile) -> Result<(), CityBuilderError> {
        let s = common_json::to_string_pretty(replay)
            .map_err(|e| CityBuilderError::Io(e.to_string()))?;
        std::fs::write(path, s).map_err(|e| CityBuilderError::Io(e.to_string()))
    }

    pub fn load(path: &Path) -> Result<ReplayFile, CityBuilderError> {
        let s = std::fs::read_to_string(path).map_err(|e| CityBuilderError::Io(e.to_string()))?;
        common_json::from_str(&s).map_err(|e| CityBuilderError::InvalidScenario(e.to_string()))
    }
}
