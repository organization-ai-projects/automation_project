use crate::diagnostics::backend_error::BackendError;
use crate::scenario::scenario::Scenario;

pub struct ScenarioLoader;

impl ScenarioLoader {
    pub fn load_from_str(data: &str) -> Result<Scenario, BackendError> {
        common_json::from_str(data).map_err(|e| BackendError::InvalidScenario(e.to_string()))
    }

    pub fn load_from_file(path: &str) -> Result<Scenario, BackendError> {
        let data = std::fs::read_to_string(path).map_err(|e| BackendError::Io(e.to_string()))?;
        Self::load_from_str(&data)
    }
}
