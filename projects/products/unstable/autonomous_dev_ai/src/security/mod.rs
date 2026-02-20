// projects/products/unstable/autonomous_dev_ai/src/security/mod.rs

//! Security, identity, authorization, and policy governance.
//!
//! Provides actor identity propagation, fine-grained authz checks, versioned/signed
//! policy packs, and privilege-escalation detection for all autonomous actions.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// ─── Actor Identity ──────────────────────────────────────────────────────────

/// Roles an actor can hold.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorRole {
    ReadOnly,
    Developer,
    Reviewer,
    Operator,
    Admin,
}

/// Identity of the actor driving autonomous actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorIdentity {
    pub id: String,
    pub roles: Vec<ActorRole>,
    pub run_id: String,
}

impl ActorIdentity {
    pub fn new(id: impl Into<String>, roles: Vec<ActorRole>, run_id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            roles,
            run_id: run_id.into(),
        }
    }

    pub fn has_role(&self, role: &ActorRole) -> bool {
        self.roles.contains(role)
    }
}

impl Default for ActorIdentity {
    fn default() -> Self {
        Self::new(
            "autonomous_dev_ai",
            vec![ActorRole::Developer],
            "default_run",
        )
    }
}

// ─── Authorization Decision ───────────────────────────────────────────────────

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

// ─── Authorization Engine ────────────────────────────────────────────────────

/// Fine-grained authorization engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthzEngine {
    /// Minimum role required per action name.
    action_role_requirements: HashMap<String, ActorRole>,
    /// Actions that require explicit escalation approval.
    privileged_actions: HashSet<String>,
}

impl AuthzEngine {
    pub fn new() -> Self {
        let mut action_role_requirements = HashMap::new();
        action_role_requirements.insert("read_file".to_string(), ActorRole::ReadOnly);
        action_role_requirements.insert("search_code".to_string(), ActorRole::ReadOnly);
        action_role_requirements.insert("run_tests".to_string(), ActorRole::Developer);
        action_role_requirements.insert("apply_patch".to_string(), ActorRole::Developer);
        action_role_requirements.insert("format_code".to_string(), ActorRole::Developer);
        action_role_requirements.insert("git_commit".to_string(), ActorRole::Developer);
        action_role_requirements.insert("git_branch".to_string(), ActorRole::Developer);
        action_role_requirements.insert("create_pr".to_string(), ActorRole::Developer);
        action_role_requirements
            .insert("generate_pr_description".to_string(), ActorRole::Developer);
        action_role_requirements.insert("merge_pr".to_string(), ActorRole::Reviewer);
        action_role_requirements.insert("deploy".to_string(), ActorRole::Operator);
        action_role_requirements.insert("modify_policy".to_string(), ActorRole::Admin);

        let mut privileged_actions = HashSet::new();
        privileged_actions.insert("deploy".to_string());
        privileged_actions.insert("modify_policy".to_string());
        privileged_actions.insert("delete_branch".to_string());

        Self {
            action_role_requirements,
            privileged_actions,
        }
    }

    /// Check whether the actor is authorized to perform the given action.
    pub fn check(&self, actor: &ActorIdentity, action: &str) -> AuthzDecision {
        // Escalation check first
        if self.privileged_actions.contains(action) {
            return AuthzDecision::RequiresEscalation {
                required_role: format!("{action}_approver"),
            };
        }

        match self.action_role_requirements.get(action) {
            None => {
                // Unknown action — deny by default
                AuthzDecision::Deny {
                    reason: format!("Action '{action}' is not in the authorization registry"),
                }
            }
            Some(required_role) => {
                let role_order = Self::role_level(required_role);
                let actor_level = actor
                    .roles
                    .iter()
                    .map(Self::role_level)
                    .max()
                    .unwrap_or(0);

                if actor_level >= role_order {
                    AuthzDecision::Allow
                } else {
                    AuthzDecision::Deny {
                        reason: format!(
                            "Action '{action}' requires role {:?}; actor '{}' does not have it",
                            required_role, actor.id
                        ),
                    }
                }
            }
        }
    }

    fn role_level(role: &ActorRole) -> usize {
        match role {
            ActorRole::ReadOnly => 1,
            ActorRole::Developer => 2,
            ActorRole::Reviewer => 3,
            ActorRole::Operator => 4,
            ActorRole::Admin => 5,
        }
    }
}

impl Default for AuthzEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Policy Pack ─────────────────────────────────────────────────────────────

/// A versioned, signable policy pack loaded at runtime.
///
/// The `forbidden_patterns` list uses case-insensitive substring matching as a
/// first-pass guard. For production hardening, pattern entries should be kept
/// canonical (single-space, lowercase) and supplemented by the `PolicyEngine`'s
/// structural command parser for robust bypass prevention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyPack {
    pub version: String,
    pub forbidden_patterns: Vec<String>,
    pub allowed_tools: Vec<String>,
    /// Optional SHA-256 hex digest of the serialized forbidden_patterns + allowed_tools.
    pub signature: Option<String>,
}

impl PolicyPack {
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
            forbidden_patterns: vec!["rm -rf".to_string(), "/etc/".to_string(), "sudo ".to_string()],
            allowed_tools: vec![
                "read_file".to_string(),
                "search_code".to_string(),
                "apply_patch".to_string(),
                "run_tests".to_string(),
                "format_code".to_string(),
                "git_commit".to_string(),
                "git_branch".to_string(),
                "create_pr".to_string(),
                "generate_pr_description".to_string(),
            ],
            signature: None,
        }
    }

    /// Compute a deterministic content fingerprint used to detect accidental tampering.
    ///
    /// Uses FNV-1a 64-bit (stable, deterministic, pure arithmetic) for content
    /// integrity verification only — this is **not** a cryptographic signature.
    pub fn fingerprint(&self) -> String {
        // FNV-1a 64-bit offset basis and prime (stable across platforms/versions).
        const OFFSET_BASIS: u64 = 14695981039346656037;
        const PRIME: u64 = 1099511628211;

        let mut hash: u64 = OFFSET_BASIS;
        let feed = |hash: &mut u64, bytes: &[u8]| {
            for &b in bytes {
                *hash ^= b as u64;
                *hash = hash.wrapping_mul(PRIME);
            }
        };

        feed(&mut hash, self.version.as_bytes());
        for p in &self.forbidden_patterns {
            feed(&mut hash, p.as_bytes());
        }
        for t in &self.allowed_tools {
            feed(&mut hash, t.as_bytes());
        }
        format!("{hash:016x}")
    }

    /// Sign by storing the fingerprint into `self.signature`.
    pub fn sign(&mut self) {
        self.signature = Some(self.fingerprint());
    }

    /// Verify that the stored signature matches the current content.
    pub fn verify(&self) -> bool {
        match &self.signature {
            None => false,
            Some(sig) => *sig == self.fingerprint(),
        }
    }
}

impl Default for PolicyPack {
    fn default() -> Self {
        let mut pack = Self::new("1.0.0");
        pack.sign();
        pack
    }
}

// ─── Audit Record ────────────────────────────────────────────────────────────

/// Audit record for an authorization or policy decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditRecord {
    pub run_id: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authz_developer_allowed() {
        let engine = AuthzEngine::new();
        let actor = ActorIdentity::new("bot", vec![ActorRole::Developer], "run-1");
        assert_eq!(engine.check(&actor, "run_tests"), AuthzDecision::Allow);
        assert_eq!(engine.check(&actor, "git_commit"), AuthzDecision::Allow);
    }

    #[test]
    fn test_authz_readonly_denied_for_commit() {
        let engine = AuthzEngine::new();
        let actor = ActorIdentity::new("bot", vec![ActorRole::ReadOnly], "run-1");
        let decision = engine.check(&actor, "git_commit");
        assert!(!decision.is_allowed());
    }

    #[test]
    fn test_authz_privileged_action_requires_escalation() {
        let engine = AuthzEngine::new();
        let actor = ActorIdentity::new("bot", vec![ActorRole::Admin], "run-1");
        let decision = engine.check(&actor, "deploy");
        assert!(matches!(decision, AuthzDecision::RequiresEscalation { .. }));
    }

    #[test]
    fn test_authz_unknown_action_denied() {
        let engine = AuthzEngine::new();
        let actor = ActorIdentity::new("bot", vec![ActorRole::Admin], "run-1");
        let decision = engine.check(&actor, "unknown_action");
        assert!(!decision.is_allowed());
    }

    #[test]
    fn test_policy_pack_sign_and_verify() {
        let mut pack = PolicyPack::new("2.0.0");
        assert!(!pack.verify(), "unsigned pack must not verify");
        pack.sign();
        assert!(pack.verify(), "signed pack must verify");
        // Tamper with the pack
        pack.forbidden_patterns.push("tampered".to_string());
        assert!(!pack.verify(), "tampered pack must not verify");
    }
}
