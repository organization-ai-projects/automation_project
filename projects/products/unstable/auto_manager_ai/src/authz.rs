// projects/products/unstable/auto_manager_ai/src/authz.rs

use crate::config::ActorIdentity;
use crate::domain::{Action, RiskLevel};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthzDecision {
    Allow,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthzRecord {
    pub action_id: String,
    pub decision: AuthzDecision,
    pub reason_code: &'static str,
    pub message: String,
}

pub fn ensure_authenticated_actor(actor: &ActorIdentity) -> Result<(), String> {
    if actor.actor_id.trim().is_empty() {
        return Err(
            "[AUTHN_ACTOR_ID_MISSING] actor_id is required before plan generation".to_string(),
        );
    }
    Ok(())
}

pub fn authorize_action(actor: &ActorIdentity, action: &Action) -> AuthzRecord {
    if actor.actor_role.eq_ignore_ascii_case("read_only") {
        return AuthzRecord {
            action_id: action.id.clone(),
            decision: AuthzDecision::Deny,
            reason_code: "AUTHZ_ROLE_READ_ONLY",
            message: "read_only actor cannot execute actions".to_string(),
        };
    }

    if !matches!(action.risk_level, RiskLevel::Low) {
        return AuthzRecord {
            action_id: action.id.clone(),
            decision: AuthzDecision::Deny,
            reason_code: "AUTHZ_RISK_LEVEL_BLOCKED",
            message: "only low-risk actions can be executed".to_string(),
        };
    }

    AuthzRecord {
        action_id: action.id.clone(),
        decision: AuthzDecision::Allow,
        reason_code: "AUTHZ_ALLOWED",
        message: "actor authorized for low-risk execution".to_string(),
    }
}
