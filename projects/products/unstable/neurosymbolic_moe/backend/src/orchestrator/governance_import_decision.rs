use crate::orchestrator::GovernanceStateDiff;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceImportDecision {
    pub allowed: bool,
    pub reasons: Vec<String>,
    pub diff: GovernanceStateDiff,
}
