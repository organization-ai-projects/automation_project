// projects/products/unstable/hospital_tycoon/backend/src/config/room_config.rs
use crate::rooms::room_kind::RoomKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomConfig {
    pub id: u32,
    pub kind: RoomKind,
    pub capacity: u32,
    pub staff_slots: u32,
}
