// projects/products/unstable/autonomous_dev_ai/src/security/security_audit_record.rs
use super::{ActorIdentity, AuthzDecision};
use crate::ids::RunId;
use serde::{Deserialize, Serialize};

/// Audit record for an authorization or policy decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditRecord {
    pub run_id: RunId,
    pub actor_id: String,
    pub action: String,
    pub decision: String,
    pub timestamp_secs: u64,
}

impl SecurityAuditRecord {
    pub fn new(actor: &ActorIdentity, action: &str, decision: &AuthzDecision) -> Self {
        let decision_str = match decision {
            AuthzDecision::Allow => "allow".to_string(),
            AuthzDecision::Deny { reason } => format!("deny: {reason}"),
            AuthzDecision::RequiresEscalation { required_role } => {
                format!("escalation_required: {required_role}")
            }
        };

        Self {
            run_id: actor.run_id.clone(),
            actor_id: actor.id.clone(),
            action: action.to_string(),
            decision: decision_str,
            timestamp_secs: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}
