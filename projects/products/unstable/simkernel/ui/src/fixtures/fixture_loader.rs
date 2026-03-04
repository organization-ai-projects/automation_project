// projects/products/unstable/simkernel/ui/src/fixtures/fixture_loader.rs
use crate::diagnostics::ui_error::UiError;
pub struct FixtureLoader;
impl FixtureLoader {
    pub fn load_report(path: &str) -> Result<String, UiError> {
        std::fs::read_to_string(path).map_err(|e| UiError::Io(e.to_string()))
    }
}
