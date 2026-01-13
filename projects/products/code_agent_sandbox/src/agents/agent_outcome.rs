// projects/products/code_agent_sandbox/src/agents/agent_outcome.rs
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentOutcome {
    pub ok: bool,
    pub iters: usize,
    pub final_score: i32,
    pub cargo_ok: bool,
    pub cargo_failures: usize,
    pub notes: Vec<String>,
    pub training_example: Option<String>,
}
