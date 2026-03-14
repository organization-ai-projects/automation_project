//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/auto_improvement_status.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutoImprovementStatus {
    pub runs_total: u64,
    pub bootstrap_entries_total: usize,
    pub last_bundle_checksum: Option<String>,
    pub last_included_entries: usize,
    pub last_train_samples: usize,
    pub last_validation_samples: usize,
}
