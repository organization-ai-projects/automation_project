// projects/products/unstable/hospital_tycoon/backend/src/patients/patient_state.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientState {
    pub id: u32,
    pub name: String,
    pub disease_name: String,
    pub tick_admitted: u64,
    pub tick_treated: Option<u64>,
    pub outcome: String,
}
