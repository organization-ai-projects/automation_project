//! projects/products/unstable/agent_engine/backend/src/engine/copy_input_step.rs
use crate::diagnostics::AgentEngineError;
use crate::engine::artifact::Artifact;
use crate::engine::execution_context::ExecutionContext;
use crate::engine::step::Step;
use crate::engine::step_result::StepResult;

pub struct CopyInputStep {
    pub input_key: String,
    pub output_key: String,
}

impl Step for CopyInputStep {
    fn name(&self) -> &'static str {
        "copy_input"
    }

    fn execute(&self, ctx: &mut ExecutionContext) -> Result<StepResult, AgentEngineError> {
        let Some(value) = ctx.task.input.get(&self.input_key).cloned() else {
            return Err(AgentEngineError::StepFailed(format!(
                "missing input key '{}'",
                self.input_key
            )));
        };
        ctx.set_output(self.output_key.clone(), value.clone());
        Ok(StepResult {
            step: self.name().to_string(),
            success: true,
            artifacts: vec![Artifact {
                name: self.output_key.clone(),
                kind: "copied_input".to_string(),
                content: value,
            }],
        })
    }
}
