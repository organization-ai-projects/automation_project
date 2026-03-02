// projects/products/unstable/hospital_tycoon/backend/src/patients/symptom.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symptom {
    pub name: String,
    pub severity: u32,
}
