//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/governance_persistence_bundle.rs
use crate::orchestrator::{GovernanceAuditEntry, GovernanceState, GovernanceStateSnapshot};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernancePersistenceBundle {
    pub state: GovernanceState,
    #[serde(default)]
    pub audit_entries: Vec<GovernanceAuditEntry>,
    #[serde(default)]
    pub snapshots: Vec<GovernanceStateSnapshot>,
}
