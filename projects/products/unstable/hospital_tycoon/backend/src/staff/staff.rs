// projects/products/unstable/hospital_tycoon/backend/src/staff/staff.rs
use crate::model::staff_id::StaffId;
use crate::staff::staff_role::StaffRole;
use crate::staff::staff_skill::StaffSkill;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Staff {
    pub id: StaffId,
    pub name: String,
    pub role: StaffRole,
    pub skill: StaffSkill,
}
