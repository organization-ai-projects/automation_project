use crate::diagnostics::error::BackendError;
use crate::scenario::scenario::Scenario;

pub struct ScenarioLoader;

impl ScenarioLoader {
    pub fn load(source: &str) -> Result<Scenario, BackendError> {
        if source == "default" {
            return Ok(Scenario::default_scenario());
        }
        common_json::from_json_str(source).map_err(|e| BackendError::Scenario(e.to_string()))
    }

    pub fn load_file(path: &str) -> Result<Scenario, BackendError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| BackendError::Scenario(format!("failed to read {path}: {e}")))?;
        Self::load(&content)
    }
}
