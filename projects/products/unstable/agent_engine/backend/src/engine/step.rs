//! projects/products/unstable/agent_engine/backend/src/engine/step.rs

use crate::{
    diagnostics::AgentEngineError,
    engine::{ExecutionContext, StepResult},
};

pub trait Step {
    fn name(&self) -> &'static str;
    fn execute(&self, ctx: &mut ExecutionContext) -> Result<StepResult, AgentEngineError>;
}
