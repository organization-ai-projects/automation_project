// projects/products/unstable/autonomy_orchestrator_ai/src/domain/gate_decision.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GateDecision {
    pub gate: String,
    pub status: String,
    pub passed: bool,
    pub reason_code: Option<String>,
}
