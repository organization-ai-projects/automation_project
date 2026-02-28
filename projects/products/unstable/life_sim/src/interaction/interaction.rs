use crate::interaction::interaction_kind::InteractionKind;
use crate::model::agent_id::AgentId;
use crate::time::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    pub kind: InteractionKind,
    pub initiator: AgentId,
    pub target: AgentId,
    pub tick: Tick,
}
