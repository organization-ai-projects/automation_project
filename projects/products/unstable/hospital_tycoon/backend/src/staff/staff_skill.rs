// projects/products/unstable/hospital_tycoon/backend/src/staff/staff_skill.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffSkill {
    pub level: u32,
}
