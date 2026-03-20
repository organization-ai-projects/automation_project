//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/governance_audit_entry.rs
use crate::orchestrator::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceAuditEntry {
    pub version: Version,
    pub checksum: String,
    pub reason: String,
}
