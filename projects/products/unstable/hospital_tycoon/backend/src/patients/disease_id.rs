// projects/products/unstable/hospital_tycoon/backend/src/patients/disease_id.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DiseaseId(pub String);

impl DiseaseId {
    pub fn new(v: impl Into<String>) -> Self {
        Self(v.into())
    }
}
