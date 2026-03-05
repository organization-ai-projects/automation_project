use crate::diagnostics::backend_error::BackendError;
use crate::scenarios::scenario::Scenario;

pub struct ScenarioValidator;

impl ScenarioValidator {
    pub fn validate(scenario: &Scenario) -> Result<(), BackendError> {
        if scenario.pack_kind.is_empty() {
            return Err(BackendError::InvalidScenario(
                "pack_kind is empty".to_string(),
            ));
        }
        if scenario.ticks == 0 {
            return Err(BackendError::InvalidScenario(
                "ticks must be > 0".to_string(),
            ));
        }
        Ok(())
    }
}
