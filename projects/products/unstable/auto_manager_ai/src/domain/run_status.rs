// projects/products/unstable/auto_manager_ai/src/domain/run_status.rs

use serde::{Deserialize, Serialize};

/// Status of the run
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RunStatus {
    Success,
    Failure,
}
