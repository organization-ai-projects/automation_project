//! projects/products/unstable/neurosymbolic_moe/backend/src/skip_counters.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkipCounters {
    pub min_dataset_entries_total: u64,
    pub min_success_ratio_total: u64,
    pub min_average_score_total: u64,
    pub human_review_required_total: u64,
    pub duplicate_bundle_total: u64,
}
