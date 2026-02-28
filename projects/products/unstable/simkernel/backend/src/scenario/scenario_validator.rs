#![allow(dead_code)]
use crate::diagnostics::error::SimError;
use crate::scenario::scenario::Scenario;

pub struct ScenarioValidator;

impl ScenarioValidator {
    pub fn validate(scenario: &Scenario) -> Result<(), SimError> {
        if scenario.pack_kind.is_empty() {
            return Err(SimError::InvalidScenario("pack_kind is empty".to_string()));
        }
        if scenario.ticks == 0 {
            return Err(SimError::InvalidScenario("ticks must be > 0".to_string()));
        }
        Ok(())
    }
}
