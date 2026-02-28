use crate::model::agent_id::AgentId;
use crate::model::room_id::RoomId;
use crate::needs::NeedsState;
use crate::relations::{MemoryLog, Relationship};
use crate::schedule::Schedule;
use crate::traits::TraitProfile;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: AgentId,
    pub name: String,
    pub room: RoomId,
    pub needs: NeedsState,
    pub traits: TraitProfile,
    pub schedule: Schedule,
    pub relationships: BTreeMap<AgentId, Relationship>,
    pub memory: MemoryLog,
}
