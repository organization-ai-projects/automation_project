use crate::contract::rule_spec::RuleSpec;
use crate::diagnostics::backend_error::BackendError;

pub struct RuleValidator;

impl RuleValidator {
    pub fn validate(rule: &RuleSpec) -> Result<(), BackendError> {
        if rule.id.trim().is_empty() {
            return Err(BackendError::Validation(
                "rule id must not be empty".to_string(),
            ));
        }
        if rule.description.trim().is_empty() {
            return Err(BackendError::Validation(
                "rule description must not be empty".to_string(),
            ));
        }
        Ok(())
    }
}
