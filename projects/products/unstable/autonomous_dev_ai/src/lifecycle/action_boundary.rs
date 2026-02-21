//projects/products/unstable/autonomous_dev_ai/src/lifecycle/action_boundary.rs
use serde::{Deserialize, Serialize};

use crate::lifecycle::CompensationKind;

/// A recorded action boundary used by the rollback manager.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionBoundary {
    pub action_name: String,
    pub compensation: CompensationKind,
    pub timestamp_secs: u64,
}
