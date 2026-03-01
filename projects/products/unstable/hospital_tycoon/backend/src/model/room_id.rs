// projects/products/unstable/hospital_tycoon/backend/src/model/room_id.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RoomId(pub u32);

impl RoomId {
    pub fn new(v: u32) -> Self {
        Self(v)
    }
}
