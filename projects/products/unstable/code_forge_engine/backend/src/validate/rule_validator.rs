// projects/products/unstable/code_forge_engine/backend/src/validate/rule_validator.rs
use crate::contract::rule_spec::RuleSpec;
use crate::diagnostics::error::ForgeError;

pub struct RuleValidator;

impl RuleValidator {
    pub fn validate(rule: &RuleSpec) -> Result<(), ForgeError> {
        if rule.id.is_empty() {
            return Err(ForgeError::Validation(
                "rule id must not be empty".to_string(),
            ));
        }
        Ok(())
    }
}
