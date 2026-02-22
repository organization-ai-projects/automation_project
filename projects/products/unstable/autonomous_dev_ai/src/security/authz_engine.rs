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

    pub fn check_with_args(
        &self,
        actor: &ActorIdentity,
        action: &str,
        args: &[String],
    ) -> AuthzDecision {
        let base = self.check(actor, action);
        if !base.is_allowed() {
            return base;
        }

        if action == "read_file" && args.iter().any(|arg| is_unsafe_file_path(arg)) {
            return AuthzDecision::Deny {
                reason: "read_file attempted to access unsafe path".to_string(),
            };
        }

        if matches!(action, "git_commit" | "git_branch" | "run_tests")
            && args.iter().any(|arg| is_forbidden_git_flag(arg))
        {
            return AuthzDecision::Deny {
                reason: "git operation contains forbidden flags".to_string(),
            };
        }

        if matches!(action, "create_pr" | "generate_pr_description")
            && !std::env::var("AUTONOMOUS_ALLOW_EXTERNAL_ACTIONS")
                .ok()
                .map(|v| v.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
        {
            return AuthzDecision::Deny {
                reason: "external actions require AUTONOMOUS_ALLOW_EXTERNAL_ACTIONS=true"
                    .to_string(),
            };
        }

        AuthzDecision::Allow
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

fn is_unsafe_file_path(path: &str) -> bool {
    let normalized = path.trim().replace('\\', "/");
    normalized.starts_with("/etc/")
        || normalized.contains("../")
        || normalized.starts_with("/proc/")
        || normalized.starts_with("/sys/")
}

fn is_forbidden_git_flag(flag: &str) -> bool {
    matches!(
        flag,
        "--force" | "-f" | "--force-with-lease" | "--mirror" | "--delete" | "-D"
    )
}

impl Default for AuthzEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{ActorId, RunId};

    fn actor_with_role(role: ActorRole) -> ActorIdentity {
        ActorIdentity::new(
            ActorId::new("security-test").expect("valid actor id"),
            vec![role],
            RunId::new("security-test-run").expect("valid run id"),
        )
    }

    #[test]
    fn denies_unsafe_read_file_paths() {
        let engine = AuthzEngine::new();
        let actor = actor_with_role(ActorRole::Developer);
        let decision = engine.check_with_args(&actor, "read_file", &[String::from("/etc/passwd")]);
        assert!(matches!(decision, AuthzDecision::Deny { .. }));
    }

    #[test]
    fn denies_forbidden_git_flags() {
        let engine = AuthzEngine::new();
        let actor = actor_with_role(ActorRole::Developer);
        let decision = engine.check_with_args(
            &actor,
            "run_tests",
            &[
                String::from("git"),
                String::from("push"),
                String::from("--force"),
            ],
        );
        assert!(matches!(decision, AuthzDecision::Deny { .. }));
    }

    #[test]
    fn denies_external_actions_without_opt_in() {
        let engine = AuthzEngine::new();
        let actor = actor_with_role(ActorRole::Developer);
        // SAFETY: test controls process env for this case.
        unsafe { std::env::remove_var("AUTONOMOUS_ALLOW_EXTERNAL_ACTIONS") };
        let decision = engine.check_with_args(&actor, "create_pr", &[]);
        assert!(matches!(decision, AuthzDecision::Deny { .. }));
    }
}
