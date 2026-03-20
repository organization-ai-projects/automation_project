use crate::orchestrator::{GovernanceAuditEntry, Version};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceAuditTrail {
    pub current_version: Version,
    pub current_checksum: Option<String>,
    pub entries: Vec<GovernanceAuditEntry>,
}
