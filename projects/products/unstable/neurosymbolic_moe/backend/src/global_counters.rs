//! projects/products/unstable/neurosymbolic_moe/backend/src/global_counters.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalCounters {
    pub runs_total: u64,
    #[serde(default)]
    pub bootstrap_entries_total: usize,
    pub build_failures_total: u64,
}
