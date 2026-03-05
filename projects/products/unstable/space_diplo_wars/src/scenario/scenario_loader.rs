use std::path::Path;

use crate::diagnostics::error::SpaceDiploWarsError;

use super::scenario::Scenario;

pub struct ScenarioLoader;

impl ScenarioLoader {
    /// Load and parse a scenario from a JSON or RON file.
    pub fn load_from_file(path: &Path) -> Result<Scenario, SpaceDiploWarsError> {
        let content = std::fs::read_to_string(path)?;
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("ron") => ron::from_str(&content)
                .map_err(|e| SpaceDiploWarsError::InvalidScenario(e.to_string())),
            _ => serde_json::from_str(&content).map_err(SpaceDiploWarsError::from),
        }
    }
}
