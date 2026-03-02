// projects/products/unstable/hospital_tycoon/backend/src/patients/disease.rs
use crate::patients::disease_id::DiseaseId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Disease {
    pub id: DiseaseId,
    pub name: String,
    pub severity: u32,
}
