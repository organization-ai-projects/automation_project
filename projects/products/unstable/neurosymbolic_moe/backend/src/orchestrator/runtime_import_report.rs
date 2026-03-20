//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/runtime_import_report.rs
use common_time::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeImportReport {
    pub imported_at_epoch_seconds: Timestamp,
    pub runtime_schema_version: u32,
    pub released_expired_leases: Timestamp,
    pub observed_dead_letter_events: u64,
    pub pending_events_after_import: usize,
    pub leased_events_after_import: usize,
    pub dead_letter_events_after_import: usize,
    pub runtime_checksum_after_import: String,
}
