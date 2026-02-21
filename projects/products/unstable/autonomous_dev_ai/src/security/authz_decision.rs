//projects/products/unstable/autonomous_dev_ai/src/security/authz_decision.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthzDecision {
    Allow,
    Deny { reason: String },
    RequiresEscalation { required_role: String },
}

impl AuthzDecision {
    pub fn is_allowed(&self) -> bool {
        matches!(self, AuthzDecision::Allow)
    }
}
