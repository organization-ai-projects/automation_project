// projects/products/unstable/hospital_tycoon/backend/src/config/disease_config.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiseaseConfig {
    pub id: String,
    pub name: String,
    pub severity: u32,
}
