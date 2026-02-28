use super::{ApprovalRule, Budget, CapabilitySet};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicySnapshot {
    pub snapshot_id: String,
    pub created_at: String,
    pub allowed_capabilities: CapabilitySet,
    pub budget: Budget,
    pub rules: Vec<ApprovalRule>,
}
