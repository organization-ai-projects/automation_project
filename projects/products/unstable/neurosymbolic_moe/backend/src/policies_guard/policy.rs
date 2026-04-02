//! projects/products/unstable/neurosymbolic_moe/backend/src/policies_guard/mod.rs
use protocol::ProtocolId;
use serde::{Deserialize, Serialize};

use crate::policies_guard::PolicyType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: ProtocolId,
    pub name: String,
    pub description: String,
    pub policy_type: PolicyType,
    pub active: bool,
}
