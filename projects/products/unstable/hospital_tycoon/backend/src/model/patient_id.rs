// projects/products/unstable/hospital_tycoon/backend/src/model/patient_id.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PatientId(pub u32);

impl PatientId {
    pub fn new(v: u32) -> Self {
        Self(v)
    }
}
