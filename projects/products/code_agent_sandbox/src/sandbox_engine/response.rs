// projects/products/code_agent_sandbox/src/engine/response.rs
use serde::Serialize;

use crate::{
    actions::ActionResult, agents::AgentOutcome, sandbox_engine::WorkspaceMode, score::ScoreSummary,
};

//replace run_id issue 67
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Response {
    pub(crate) run_id: String,
    pub(crate) workspace_mode: WorkspaceMode,
    pub(crate) work_root: String,
    pub(crate) results: Vec<ActionResult>,
    pub(crate) score: ScoreSummary,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) agent_outcome: Option<AgentOutcome>,
}

impl Response {
    pub(crate) fn new(
        run_id: String,
        workspace_mode: WorkspaceMode,
        work_root: String,
        results: Vec<ActionResult>,
        score: ScoreSummary,
        agent_outcome: Option<AgentOutcome>,
    ) -> Self {
        Self {
            run_id,
            workspace_mode,
            work_root,
            results,
            score,
            agent_outcome,
        }
    }
}
