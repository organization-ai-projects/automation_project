//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/governance_audit_entry.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceAuditEntry {
    pub version: u64,
    pub checksum: String,
    pub reason: String,
}
