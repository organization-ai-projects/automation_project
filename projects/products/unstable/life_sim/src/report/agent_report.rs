use crate::model::agent_id::AgentId;
use crate::needs::NeedsState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentReport {
    pub agent_id: AgentId,
    pub name: String,
    pub final_needs: NeedsState,
    pub memory_count: usize,
    pub actions_taken: u64,
}
