use crate::orchestrator::{GovernanceAuditEntry, GovernanceState, GovernanceStateSnapshot};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernancePersistenceBundle {
    pub state: GovernanceState,
    pub audit_entries: Vec<GovernanceAuditEntry>,
    pub snapshots: Vec<GovernanceStateSnapshot>,
}
