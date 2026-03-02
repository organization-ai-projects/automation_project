// projects/products/unstable/hospital_tycoon/backend/src/config/sim_config.rs
use crate::rooms::room_kind::RoomKind;
use crate::staff::staff_role::StaffRole;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomConfig {
    pub id: u32,
    pub kind: RoomKind,
    pub capacity: u32,
    pub staff_slots: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffConfig {
    pub id: u32,
    pub name: String,
    pub role: StaffRole,
    pub skill_level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiseaseConfig {
    pub id: String,
    pub name: String,
    pub severity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimConfig {
    pub name: String,
    pub seed: u64,
    pub ticks: u64,
    pub rooms: Vec<RoomConfig>,
    pub staff: Vec<StaffConfig>,
    pub diseases: Vec<DiseaseConfig>,
    pub initial_budget: i64,
    pub initial_reputation: u32,
    pub patient_spawn_rate: u64,
}

impl SimConfig {
    pub fn load(path: &std::path::Path) -> Result<Self, crate::diagnostics::error::AppError> {
        let data = std::fs::read_to_string(path)
            .map_err(|e| crate::diagnostics::error::AppError::Io(e.to_string()))?;
        serde_json::from_str(&data)
            .map_err(|e| crate::diagnostics::error::AppError::Config(e.to_string()))
    }
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            seed: 42,
            ticks: 50,
            rooms: vec![RoomConfig {
                id: 1,
                kind: RoomKind::Treatment,
                capacity: 5,
                staff_slots: 2,
            }],
            staff: vec![StaffConfig {
                id: 1,
                name: "Dr. Default".to_string(),
                role: StaffRole::Doctor,
                skill_level: 3,
            }],
            diseases: vec![DiseaseConfig {
                id: "cold".to_string(),
                name: "Common Cold".to_string(),
                severity: 1,
            }],
            initial_budget: 10000,
            initial_reputation: 50,
            patient_spawn_rate: 5,
        }
    }
}
