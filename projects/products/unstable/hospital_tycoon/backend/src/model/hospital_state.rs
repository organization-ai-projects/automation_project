// projects/products/unstable/hospital_tycoon/backend/src/model/hospital_state.rs
use crate::economy::budget::Budget;
use crate::model::patient_id::PatientId;
use crate::model::room_id::RoomId;
use crate::model::staff_id::StaffId;
use crate::patients::patient::Patient;
use crate::patients::patient_state::PatientState;
use crate::reputation::reputation::Reputation;
use crate::rooms::room::Room;
use crate::rooms::room_queue::RoomQueue;
use crate::staff::staff::Staff;
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HospitalState {
    pub patients: BTreeMap<PatientId, Patient>,
    pub treated_patients: Vec<PatientState>,
    pub rooms: BTreeMap<RoomId, Room>,
    pub room_queues: BTreeMap<RoomId, RoomQueue>,
    pub staff: BTreeMap<StaffId, Staff>,
    pub budget: Budget,
    pub reputation: Reputation,
    pub tick: Tick,
    pub next_patient_id: u32,
    /// Patients waiting to be triaged into a room
    pub waiting_patients: Vec<PatientId>,
}

impl HospitalState {
    pub fn patients_treated(&self) -> u32 {
        self.treated_patients.len() as u32
    }
}
