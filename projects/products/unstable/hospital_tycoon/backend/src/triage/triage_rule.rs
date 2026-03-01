// projects/products/unstable/hospital_tycoon/backend/src/triage/triage_rule.rs
use crate::patients::disease_id::DiseaseId;
use crate::rooms::room_kind::RoomKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriageRule {
    pub disease_id: DiseaseId,
    pub priority: u32,
    pub target_room_kind: RoomKind,
}
