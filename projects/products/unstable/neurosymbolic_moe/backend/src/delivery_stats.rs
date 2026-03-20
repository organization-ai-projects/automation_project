//! projects/products/unstable/neurosymbolic_moe/backend/src/delivery_stats.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeliveryStats {
    pub delivery_attempts_total: u64,
    pub delivery_failures_total: u64,
    pub acknowledged_total: u64,
    #[serde(default)]
    pub dead_letter_total: u64,
}
