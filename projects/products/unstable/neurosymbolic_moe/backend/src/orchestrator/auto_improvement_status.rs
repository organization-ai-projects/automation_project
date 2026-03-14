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
    pub skipped_min_dataset_entries_total: u64,
    pub skipped_min_success_ratio_total: u64,
    pub skipped_min_average_score_total: u64,
    pub skipped_human_review_required_total: u64,
    pub skipped_duplicate_bundle_total: u64,
    pub build_failures_total: u64,
    pub last_skip_reason: Option<String>,
    pub trainer_trigger_delivery_attempts_total: u64,
    pub trainer_trigger_delivery_failures_total: u64,
    pub trainer_trigger_acknowledged_total: u64,
}
