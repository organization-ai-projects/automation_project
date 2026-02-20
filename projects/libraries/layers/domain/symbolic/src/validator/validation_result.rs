// projects/libraries/layers/domain/symbolic/src/validator/validation_result.rs
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub reason: Option<String>,
    pub suggested_fix: Option<String>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            reason: None,
            suggested_fix: None,
        }
    }

    pub fn invalid(errors: Vec<String>) -> Self {
        Self {
            is_valid: false,
            errors,
            warnings: Vec::new(),
            reason: None,
            suggested_fix: None,
        }
    }

    pub fn with_warnings(mut self, warnings: Vec<String>) -> Self {
        self.warnings = warnings;
        self
    }

    pub fn with_reason(mut self, reason: String) -> Self {
        self.reason = Some(reason);
        self
    }

    pub fn with_suggested_fix(mut self, suggested_fix: String) -> Self {
        self.suggested_fix = Some(suggested_fix);
        self
    }
}
