// projects/products/unstable/hospital_tycoon/backend/src/report/tick_report.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickReport {
    pub tick: u64,
    pub patients_in_queue: usize,
    pub patients_treated: u32,
    pub budget_balance: i64,
    pub reputation_score: u32,
}
