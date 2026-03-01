#![allow(dead_code)]
use crate::diagnostics::error::SimError;
use crate::scenario::scenario::Scenario;

/// Loads scenarios from JSON files or strings.
pub struct ScenarioLoader;

impl ScenarioLoader {
    pub fn load_from_str(data: &str) -> Result<Scenario, SimError> {
        serde_json::from_str(data).map_err(|e| SimError::InvalidScenario(e.to_string()))
    }

    pub fn load_from_file(path: &str) -> Result<Scenario, SimError> {
        let data = std::fs::read_to_string(path).map_err(|e| SimError::Io(e.to_string()))?;
        Self::load_from_str(&data)
    }
}
