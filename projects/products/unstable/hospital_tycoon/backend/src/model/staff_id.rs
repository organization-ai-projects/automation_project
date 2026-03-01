// projects/products/unstable/hospital_tycoon/backend/src/model/staff_id.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct StaffId(pub u32);

impl StaffId {
    pub fn new(v: u32) -> Self {
        Self(v)
    }
}
