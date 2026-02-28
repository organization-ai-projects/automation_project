// projects/products/unstable/autonomy_orchestrator_ai/src/domain/rollout_step.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutPhase {
    Canary,
    Partial,
    Full,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloutStep {
    pub phase: RolloutPhase,
    pub reason_code: String,
    pub timestamp_unix_secs: u64,
}
