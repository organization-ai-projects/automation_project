//! projects/products/unstable/agent_engine/backend/src/engine/log_step.rs

use crate::{
    diagnostics::AgentEngineError,
    engine::{Artifact, ExecutionContext, Step, StepResult},
};

pub struct LogStep {
    pub message: String,
}

impl Step for LogStep {
    fn name(&self) -> &'static str {
        "log"
    }

    fn execute(&self, ctx: &mut ExecutionContext) -> Result<StepResult, AgentEngineError> {
        ctx.append_log(self.message.clone());
        Ok(StepResult {
            step: self.name().to_string(),
            success: true,
            artifacts: vec![Artifact {
                name: "log_entry".to_string(),
                kind: "runtime_log".to_string(),
                content: self.message.clone(),
            }],
        })
    }
}
