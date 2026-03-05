// projects/products/unstable/hospital_tycoon/ui/src/transport/run_report_dto.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunReportDto {
    pub seed: u64,
    pub scenario_name: String,
    pub total_ticks: u64,
    pub patients_treated: u32,
    pub patients_died: u32,
    pub final_budget: i64,
    pub final_reputation: u32,
    pub event_count: usize,
    pub run_hash: String,
}
