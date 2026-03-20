use crate::orchestrator::GovernanceState;
use crate::orchestrator::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceStateSnapshot {
    pub version: Version,
    pub reason: String,
    pub state: GovernanceState,
}
