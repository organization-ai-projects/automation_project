// projects/products/unstable/hospital_tycoon/backend/src/staff/staff_engine.rs
use crate::model::staff_id::StaffId;
use crate::staff::staff::Staff;
use std::collections::BTreeMap;

pub struct StaffEngine;

impl StaffEngine {
    pub fn available_count(staff: &BTreeMap<StaffId, Staff>) -> usize {
        staff.len()
    }
}
