use std::path::Path;

use crate::diagnostics::error::Error;
use crate::scenario::scenario::Scenario;

pub struct ScenarioLoader;

impl ScenarioLoader {
    pub fn load_from_file(path: &Path) -> Result<Scenario, Error> {
        let data =
            std::fs::read_to_string(path).map_err(|e| Error::InvalidScenario(e.to_string()))?;
        Self::load_from_str(&data)
    }

    pub fn load_from_str(data: &str) -> Result<Scenario, Error> {
        common_json::from_str(data).map_err(|e| Error::InvalidScenario(e.to_string()))
    }
}
