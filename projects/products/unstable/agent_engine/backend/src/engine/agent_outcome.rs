//! projects/products/unstable/agent_engine/backend/src/engine/agent_outcome.rs
use std::collections;

use crate::engine::step_result;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct AgentOutcome {
    pub task_id: String,
    pub success: bool,
    pub step_results: Vec<step_result::StepResult>,
    pub output: collections::BTreeMap<String, String>,
    pub logs: Vec<String>,
}
