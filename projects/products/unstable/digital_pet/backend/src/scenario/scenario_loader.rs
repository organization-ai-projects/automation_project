// projects/products/unstable/digital_pet/backend/src/scenario/scenario_loader.rs
use crate::diagnostics::error::AppError;
use crate::scenario::scenario::Scenario;
use std::path::Path;

pub struct ScenarioLoader;

impl ScenarioLoader {
    pub fn load(path: &Path) -> Result<Scenario, AppError> {
        if !path.exists() {
            return Ok(Scenario::default());
        }
        let data = std::fs::read_to_string(path).map_err(|e| AppError::Io(e.to_string()))?;
        serde_json::from_str(&data).map_err(|e| AppError::Config(e.to_string()))
    }
}
