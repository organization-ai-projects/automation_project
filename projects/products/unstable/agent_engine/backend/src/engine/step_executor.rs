//! projects/products/unstable/agent_engine/backend/src/engine/step_executor.rs

use crate::{
    diagnostics::AgentEngineError,
    engine::{ExecutionContext, Step, StepResult},
};

pub struct StepExecutor;

impl StepExecutor {
    pub fn run(
        ctx: &mut ExecutionContext,
        steps: &[Box<dyn Step>],
    ) -> Result<Vec<StepResult>, AgentEngineError> {
        let mut out = Vec::new();
        for step in steps {
            let result = step.execute(ctx)?;
            out.push(result);
        }
        Ok(out)
    }
}
