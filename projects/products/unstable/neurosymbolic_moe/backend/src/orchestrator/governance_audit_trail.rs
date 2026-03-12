use crate::orchestrator::GovernanceAuditEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceAuditTrail {
    pub current_version: u64,
    pub current_checksum: Option<String>,
    pub entries: Vec<GovernanceAuditEntry>,
}
