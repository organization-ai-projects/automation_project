// projects/products/unstable/autonomy_orchestrator_ai/src/domain/policy_gate_status.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyGateStatus {
    Allow,
    Deny,
    Unknown,
}
