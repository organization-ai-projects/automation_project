// projects/products/code_agent_sandbox/src/agents/agent_outcome.rs
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AgentOutcome {
    pub(crate) ok: bool,
    pub(crate) iters: usize,
    pub(crate) final_score: i32,
    pub(crate) cargo_ok: bool,
    pub(crate) cargo_failures: usize,
    pub(crate) notes: Vec<String>,
    pub(crate) training_example: Option<String>,
}
