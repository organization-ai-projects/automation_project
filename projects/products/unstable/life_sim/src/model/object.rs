use crate::model::object_id::ObjectId;
use crate::model::room_id::RoomId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    pub id: ObjectId,
    pub name: String,
    pub room: RoomId,
    pub tags: Vec<String>,
}
