use super::scenario::Scenario;
use crate::diagnostics::error::CityBuilderError;
use std::path::Path;

pub struct ScenarioLoader;

impl ScenarioLoader {
    pub fn load(path: &Path) -> Result<Scenario, CityBuilderError> {
        let s = std::fs::read_to_string(path)
            .map_err(|e| CityBuilderError::InvalidScenario(e.to_string()))?;
        serde_json::from_str(&s)
            .map_err(|e| CityBuilderError::InvalidScenario(e.to_string()))
    }
}
