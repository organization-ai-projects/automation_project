use crate::diagnostics::error::Error;

use ai::{AiBody, Task};

pub struct TranspileValidator {
    ai: AiBody,
}

impl TranspileValidator {
    pub fn new() -> Result<Self, Error> {
        let ai = AiBody::new()?;
        Ok(Self { ai })
    }

    pub fn validate_transpilation(
        &mut self,
        rhl_source: &str,
        rust_output: &str,
    ) -> Result<String, Error> {
        let prompt = format!(
            "Validate that this Rust transpilation is correct for the given RHL source.\n\nRHL Source:\n{rhl_source}\n\nRust Output:\n{rust_output}"
        );
        let task = Task::new_code_analysis(prompt);
        let result = self.ai.solve(&task)?;
        Ok(result.output)
    }

    pub fn check_safety(&mut self, rust_code: &str) -> Result<String, Error> {
        let result = self.ai.analyze_code(rust_code)?;
        Ok(result)
    }
}
