// projects/products/code_agent_sandbox/src/engine/response.rs
use serde::Serialize;

use crate::{
    actions::ActionResult, agents::AgentOutcome, engine::WorkspaceMode, score::ScoreSummary,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub run_id: String,
    pub workspace_mode: WorkspaceMode,
    pub work_root: String,
    pub results: Vec<ActionResult>,
    pub score: ScoreSummary,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_outcome: Option<AgentOutcome>,
}
