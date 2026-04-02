use crate::diagnostics::error::UiError;

pub struct FixtureLoader;

impl FixtureLoader {
    pub fn load_scenario(name: &str) -> Result<String, UiError> {
        if name == "default" {
            return Ok("default".to_string());
        }
        let content = std::fs::read_to_string(name)
            .map_err(|e| UiError::State(format!("failed to load fixture {name}: {e}")))?;
        Ok(content)
    }
}
