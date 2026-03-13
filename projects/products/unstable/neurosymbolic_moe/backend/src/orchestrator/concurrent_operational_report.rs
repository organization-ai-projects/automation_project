//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/concurrent_operational_report.rs
use serde::{Deserialize, Serialize};

use crate::orchestrator::{ConcurrentLockMetrics, OperationalReport};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrentOperationalReport {
    pub pipeline: OperationalReport,
    pub lock_metrics: ConcurrentLockMetrics,
    pub lock_contention_rate: f64,
    pub lock_timeout_rate: f64,
}
