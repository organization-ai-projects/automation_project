//! projects/products/unstable/autonomous_dev_ai/backend/src/ops/ops_alert.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpsAlert {
    pub severity: String,
    pub code: String,
    pub message: String,
}
