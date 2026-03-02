// projects/products/unstable/hospital_tycoon/backend/src/rooms/room_kind.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomKind {
    Reception,
    Diagnosis,
    Treatment,
    Recovery,
}
