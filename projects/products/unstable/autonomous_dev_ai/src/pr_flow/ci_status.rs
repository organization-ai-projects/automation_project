//projects/products/unstable/autonomous_dev_ai/src/pr_flow/ci_status.rs
use serde::{Deserialize, Serialize};

/// Status of a CI check for a given PR.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CiStatus {
    Pending,
    Passing,
    Failing,
    Unknown,
}
