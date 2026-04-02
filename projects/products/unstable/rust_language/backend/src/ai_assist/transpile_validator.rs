//! projects/products/unstable/rust_language/backend/src/ai_assist/transpile_validator.rs
use ai::{AiBody, Task};

use crate::app_error::AppError;

pub(crate) struct TranspileValidator {
    pub(crate) ai: AiBody,
}

impl TranspileValidator {
    pub(crate) fn new() -> Result<Self, AppError> {
        let ai = AiBody::new().map_err(AppError::from)?;
        Ok(Self { ai })
    }

    pub(crate) fn validate_transpilation(
        &mut self,
        rhl_source: &str,
        rust_output: &str,
    ) -> Result<String, AppError> {
        let prompt = format!(
            "Validate that this Rust transpilation is correct for the given RHL source.\n\nRHL Source:\n{rhl_source}\n\nRust Output:\n{rust_output}"
        );
        let task = Task::new_code_analysis(prompt);
        let result = self.ai.solve(&task).map_err(AppError::from)?;
        Ok(result.output)
    }

    pub(crate) fn check_safety(&mut self, rust_code: &str) -> Result<String, AppError> {
        let result = self.ai.analyze_code(rust_code).map_err(AppError::from)?;
        Ok(result)
    }
}
