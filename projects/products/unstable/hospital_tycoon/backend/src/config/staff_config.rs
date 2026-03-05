// projects/products/unstable/hospital_tycoon/backend/src/config/staff_config.rs
use crate::staff::staff_role::StaffRole;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffConfig {
    pub id: u32,
    pub name: String,
    pub role: StaffRole,
    pub skill_level: u32,
}
