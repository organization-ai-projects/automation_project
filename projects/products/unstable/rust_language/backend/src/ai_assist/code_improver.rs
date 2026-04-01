use crate::diagnostics::error::Error;

use ai::AiBody;

pub struct CodeImprover {
    ai: AiBody,
}

impl CodeImprover {
    pub fn new() -> Result<Self, Error> {
        let ai = AiBody::new()?;
        Ok(Self { ai })
    }

    pub fn improve_rhl_code(&mut self, source_code: &str) -> Result<String, Error> {
        let prompt = format!(
            "Improve this RHL source code for clarity, performance, and correctness.\n\nSource:\n{source_code}"
        );
        let task = self.ai.create_task(&prompt);
        let result = self.ai.solve(&task)?;
        Ok(result.output)
    }

    pub fn optimize_transpiled_rust(&mut self, rust_code: &str) -> Result<String, Error> {
        let instruction = "Optimize this transpiled Rust code for performance and idiomatic style.";
        let result = self.ai.refactor_code(rust_code, instruction)?;
        Ok(result)
    }
}
