// projects/products/unstable/autonomy_orchestrator_ai/src/domain/rollback_decision.rs
use serde::{Deserialize, Serialize};

use crate::domain::RolloutPhase;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDecision {
    pub triggered_at_phase: RolloutPhase,
    pub reason_code: String,
    pub timestamp_unix_secs: u64,
}
