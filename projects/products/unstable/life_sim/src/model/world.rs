use crate::model::agent::Agent;
use crate::model::agent_id::AgentId;
use crate::model::object::Object;
use crate::model::object_id::ObjectId;
use crate::model::room::Room;
use crate::model::room_id::RoomId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub rooms: BTreeMap<RoomId, Room>,
    pub objects: BTreeMap<ObjectId, Object>,
    pub agents: BTreeMap<AgentId, Agent>,
}
