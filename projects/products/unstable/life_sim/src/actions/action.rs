use crate::actions::action_cost::ActionCost;
use crate::actions::action_effect::ActionEffect;
use crate::actions::action_kind::ActionKind;
use crate::model::agent_id::AgentId;
use crate::model::object_id::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub kind: ActionKind,
    pub target_agent: Option<AgentId>,
    pub target_object: Option<ObjectId>,
    pub cost: ActionCost,
    pub effect: ActionEffect,
}
