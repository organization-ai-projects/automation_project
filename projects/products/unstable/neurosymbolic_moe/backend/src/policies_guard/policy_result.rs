//! projects/products/unstable/neurosymbolic_moe/backend/src/policies_guard/policy_result.rs
use protocol::ProtocolId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyResult {
    pub policy_id: ProtocolId,
    pub passed: bool,
    pub reason: Option<String>,
}
