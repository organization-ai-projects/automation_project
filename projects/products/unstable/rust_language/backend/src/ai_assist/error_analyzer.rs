use crate::diagnostics::error::Error;

use ai::{AiBody, Task};

pub struct ErrorAnalyzer {
    ai: AiBody,
}

impl ErrorAnalyzer {
    pub fn new() -> Result<Self, Error> {
        let ai = AiBody::new()?;
        Ok(Self { ai })
    }

    pub fn analyze_compilation_error(
        &mut self,
        source_code: &str,
        error_message: &str,
    ) -> Result<String, Error> {
        let prompt = format!(
            "Analyze this RHL compilation error and explain what went wrong.\n\nSource:\n{source_code}\n\nError:\n{error_message}"
        );
        let task = Task::new_code_analysis(prompt);
        let result = self.ai.solve(&task)?;
        Ok(result.output)
    }

    pub fn suggest_fix(&mut self, source_code: &str, error_message: &str) -> Result<String, Error> {
        let prompt = format!(
            "Suggest a fix for this RHL compilation error.\n\nSource:\n{source_code}\n\nError:\n{error_message}"
        );
        let task = Task::new_code_analysis(prompt);
        let result = self.ai.solve(&task)?;
        Ok(result.output)
    }
}
