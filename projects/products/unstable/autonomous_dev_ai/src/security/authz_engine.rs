// projects/products/unstable/autonomous_dev_ai/src/security/authz_engine.rs
use super::{ActorIdentity, ActorRole, AuthzDecision};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

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
        if self.privileged_actions.contains(action) {
            return AuthzDecision::RequiresEscalation {
                required_role: format!("{action}_approver"),
            };
        }

        match self.action_role_requirements.get(action) {
            None => AuthzDecision::Deny {
                reason: format!("Action '{action}' is not in the authorization registry"),
            },
            Some(required_role) => {
                let role_order = Self::role_level(required_role);
                let actor_level = actor.roles.iter().map(Self::role_level).max().unwrap_or(0);

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
