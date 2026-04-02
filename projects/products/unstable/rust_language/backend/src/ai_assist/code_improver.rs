//! projects/products/unstable/rust_language/backend/src/ai_assist/code_improver.rs
use ai::AiBody;

use crate::app_error::AppError;

pub(crate) struct CodeImprover {
    pub(crate) ai: AiBody,
}

impl CodeImprover {
    pub(crate) fn new() -> Result<Self, AppError> {
        let ai = AiBody::new()?;
        Ok(Self { ai })
    }

    pub(crate) fn improve_rhl_code(&mut self, source_code: &str) -> Result<String, AppError> {
        let prompt = format!(
            "Improve this RHL source code for clarity, performance, and correctness.\n\nSource:\n{source_code}"
        );
        let task = self.ai.create_task(&prompt);
        let result = self.ai.solve(&task)?;
        Ok(result.output)
    }

    pub(crate) fn optimize_transpiled_rust(&mut self, rust_code: &str) -> Result<String, AppError> {
        let instruction = "Optimize this transpiled Rust code for performance and idiomatic style.";
        let result = self.ai.refactor_code(rust_code, instruction)?;
        Ok(result)
    }
}
