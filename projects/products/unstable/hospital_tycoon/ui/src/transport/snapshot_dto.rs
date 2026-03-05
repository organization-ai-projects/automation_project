// projects/products/unstable/hospital_tycoon/ui/src/transport/snapshot_dto.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotDto {
    pub tick: u64,
    pub patient_count: usize,
    pub budget_balance: i64,
    pub reputation_score: u32,
    pub patients_treated: u32,
    pub hash: String,
    pub data_summary: String,
}
