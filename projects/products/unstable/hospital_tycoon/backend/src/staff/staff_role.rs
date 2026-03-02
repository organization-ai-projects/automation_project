// projects/products/unstable/hospital_tycoon/backend/src/staff/staff_role.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StaffRole {
    Doctor,
    Nurse,
    Receptionist,
}
