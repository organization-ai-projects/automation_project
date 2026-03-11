use crate::orchestrator::GovernanceState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceStateSnapshot {
    pub version: u64,
    pub reason: String,
    pub state: GovernanceState,
}
