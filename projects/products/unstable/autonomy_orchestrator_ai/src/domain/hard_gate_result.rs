// projects/products/unstable/autonomy_orchestrator_ai/src/domain/hard_gate_result.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardGateResult {
    pub id: String,
    pub passed: bool,
    pub reason_code: String,
}
