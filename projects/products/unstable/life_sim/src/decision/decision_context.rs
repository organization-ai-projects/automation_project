use crate::actions::Action;
use crate::model::agent_id::AgentId;
use crate::time::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionContext {
    pub agent_id: AgentId,
    pub tick: Tick,
    pub available_actions: Vec<Action>,
}
