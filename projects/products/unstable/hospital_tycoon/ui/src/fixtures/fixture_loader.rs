// projects/products/unstable/hospital_tycoon/ui/src/fixtures/fixture_loader.rs
use crate::diagnostics::error::AppError;
use std::path::Path;

pub struct FixtureLoader;

impl FixtureLoader {
    pub fn load_json(path: &Path) -> Result<serde_json::Value, AppError> {
        let data = std::fs::read_to_string(path).map_err(|e| AppError::Io(e.to_string()))?;
        serde_json::from_str(&data).map_err(|e| AppError::Io(e.to_string()))
    }
}
