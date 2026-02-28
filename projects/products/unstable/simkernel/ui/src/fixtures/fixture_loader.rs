#![allow(dead_code)]
use crate::diagnostics::error::UiError;
pub struct FixtureLoader;
impl FixtureLoader {
    pub fn load_report(path: &str) -> Result<String, UiError> {
        std::fs::read_to_string(path).map_err(|e| UiError::Io(e.to_string()))
    }
}
