// projects/products/unstable/hospital_tycoon/backend/src/rooms/room.rs
use crate::model::room_id::RoomId;
use crate::rooms::room_kind::RoomKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: RoomId,
    pub kind: RoomKind,
    pub capacity: u32,
    pub staff_slots: u32,
}
