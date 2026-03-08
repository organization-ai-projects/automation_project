//! projects/products/unstable/agent_engine/backend/src/engine/log_step.rs

use crate::{
    diagnostics::AgentEngineError,
    engine::{Artifact, ExecutionContext, Step, StepResult},
};

pub struct SetOutputStep {
    pub key: String,
    pub value: String,
}

impl Step for SetOutputStep {
    fn name(&self) -> &'static str {
        "set_output"
    }

    fn execute(&self, ctx: &mut ExecutionContext) -> Result<StepResult, AgentEngineError> {
        ctx.set_output(self.key.clone(), self.value.clone());
        Ok(StepResult {
            step: self.name().to_string(),
            success: true,
            artifacts: vec![Artifact {
                name: self.key.clone(),
                kind: "output_kv".to_string(),
                content: self.value.clone(),
            }],
        })
    }
}
