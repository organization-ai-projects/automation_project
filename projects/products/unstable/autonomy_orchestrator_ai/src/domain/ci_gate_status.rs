// projects/products/unstable/autonomy_orchestrator_ai/src/domain/ci_gate_status.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CiGateStatus {
    Success,
    Pending,
    Failure,
    Missing,
}
