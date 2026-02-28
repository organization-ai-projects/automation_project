use crate::model::object_id::ObjectId;
use crate::model::room_id::RoomId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: RoomId,
    pub name: String,
    pub objects: Vec<ObjectId>,
    pub capacity: u32,
}
