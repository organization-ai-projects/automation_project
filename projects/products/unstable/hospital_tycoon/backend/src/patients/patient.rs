// projects/products/unstable/hospital_tycoon/backend/src/patients/patient.rs
use crate::model::patient_id::PatientId;
use crate::model::room_id::RoomId;
use crate::patients::disease::Disease;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    pub id: PatientId,
    pub name: String,
    pub disease: Disease,
    pub severity: u32,
    pub assigned_room: Option<RoomId>,
    pub tick_admitted: u64,
}
