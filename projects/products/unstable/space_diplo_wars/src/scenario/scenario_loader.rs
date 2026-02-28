use std::path::Path;

use crate::diagnostics::error::SpaceDiploWarsError;

use super::scenario::Scenario;

pub struct ScenarioLoader;

impl ScenarioLoader {
    /// Load and parse a scenario from a JSON file.
    pub fn load_from_file(path: &Path) -> Result<Scenario, SpaceDiploWarsError> {
        let content = std::fs::read_to_string(path)?;
        let scenario: Scenario = serde_json::from_str(&content)?;
        Ok(scenario)
    }
}
