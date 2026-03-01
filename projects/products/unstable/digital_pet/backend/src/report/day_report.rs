// projects/products/unstable/digital_pet/backend/src/report/day_report.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayReport {
    pub day: u64,
    pub species: String,
    pub care_mistakes: usize,
    pub happiness: u32,
    pub discipline: u32,
}
