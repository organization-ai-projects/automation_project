//! projects/products/unstable/rust_language/backend/src/ai_assist/error_analyzer.rs
use crate::app_error::AppError;
use ai::{AiBody, Task};

pub(crate) struct ErrorAnalyzer {
    ai: AiBody,
}

impl ErrorAnalyzer {
    pub(crate) fn new() -> Result<Self, AppError> {
        let ai = AiBody::new().map_err(AppError::from)?;
        Ok(Self { ai })
    }

    pub(crate) fn analyze_compilation_error(
        &mut self,
        source_code: &str,
        error_message: &str,
    ) -> Result<String, AppError> {
        let prompt = format!(
            "Analyze this RHL compilation error and explain what went wrong.\n\nSource:\n{source_code}\n\nError:\n{error_message}"
        );
        let task = Task::new_code_analysis(prompt);
        let result = self.ai.solve(&task).map_err(AppError::from)?;
        Ok(result.output)
    }

    pub(crate) fn suggest_fix(
        &mut self,
        source_code: &str,
        error_message: &str,
    ) -> Result<String, AppError> {
        let prompt = format!(
            "Suggest a fix for this RHL compilation error.\n\nSource:\n{source_code}\n\nError:\n{error_message}"
        );
        let task = Task::new_code_analysis(prompt);
        let result = self.ai.solve(&task).map_err(AppError::from)?;
        Ok(result.output)
    }
}
