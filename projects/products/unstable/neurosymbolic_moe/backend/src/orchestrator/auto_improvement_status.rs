//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/auto_improvement_status.rs
use serde::{Deserialize, Serialize};

use crate::{
    delivery_stats::DeliveryStats, global_counters::GlobalCounters, skip_counters::SkipCounters,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutoImprovementStatus {
    pub global_counters: GlobalCounters,
    pub skip_counters: SkipCounters,
    pub delivery_stats: DeliveryStats,
    pub last_bundle_checksum: Option<String>,
    pub last_included_entries: usize,
    pub last_train_samples: usize,
    pub last_validation_samples: usize,
    pub last_skip_reason: Option<String>,
}
