// projects/products/unstable/auto_manager_ai/src/domain/policy.rs

use super::action::Action;
use super::policy_decision::PolicyDecision;
use super::policy_decision_type::PolicyDecisionType;

/// Policy rules for evaluating actions
#[derive(Debug, Clone)]
pub struct Policy {
    pub min_confidence: f64,
    pub _allow_repo_writes: bool,
    pub _allow_github_writes: bool,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            min_confidence: 0.6,
            _allow_repo_writes: false,   // V0: no repo writes
            _allow_github_writes: false, // V0: no GitHub writes
        }
    }
}

impl Policy {
    /// Evaluate an action against the policy
    pub fn evaluate(&self, action: &Action) -> PolicyDecision {
        // Check confidence threshold
        if action.confidence < self.min_confidence {
            return PolicyDecision {
                action_id: action.id.clone(),
                decision: PolicyDecisionType::Deny,
                reason: format!(
                    "Confidence {} below threshold {}",
                    action.confidence, self.min_confidence
                ),
            };
        }

        // Check for missing inputs
        if let Some(missing) = &action.missing_inputs
            && !missing.is_empty()
        {
            return PolicyDecision {
                action_id: action.id.clone(),
                decision: PolicyDecisionType::NeedsInput,
                reason: format!("Missing inputs: {}", missing.join(", ")),
            };
        }

        // V0: Deny all write operations by default
        if self.is_write_action(&action.action_type) {
            return PolicyDecision {
                action_id: action.id.clone(),
                decision: PolicyDecisionType::Deny,
                reason: "Write actions are forbidden in V0 (safe-by-default)".to_string(),
            };
        }

        // Allow read-only actions
        PolicyDecision {
            action_id: action.id.clone(),
            decision: PolicyDecisionType::Allow,
            reason: "Read-only action approved".to_string(),
        }
    }

    /// Check if an action type represents a write operation
    fn is_write_action(&self, action_type: &str) -> bool {
        matches!(
            action_type,
            "create_issue"
                | "create_branch"
                | "open_draft_pr"
                | "post_pr_comment"
                | "commit"
                | "push"
                | "merge"
                | "force_push"
                | "write_file"
                | "delete_file"
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::policy::Policy;
    use crate::domain::policy_decision_type::PolicyDecisionType;
    use crate::tests::test_helpers::build_test_action;

    #[test]
    fn test_policy_default_deny_writes() {
        let policy = Policy::default();
        let action = build_test_action("create_issue", 0.9);

        let decision = policy.evaluate(&action);

        assert_eq!(decision.decision, PolicyDecisionType::Deny);
        assert!(decision.reason.contains("forbidden") || decision.reason.contains("V0"));
    }

    #[test]
    fn test_policy_allows_read_only() {
        let policy = Policy::default();
        let action = build_test_action("analyze_repository", 0.9);

        let decision = policy.evaluate(&action);

        assert_eq!(decision.decision, PolicyDecisionType::Allow);
    }

    #[test]
    fn test_policy_confidence_threshold() {
        let policy = Policy::default();
        let action = build_test_action("analyze_repository", 0.3);

        let decision = policy.evaluate(&action);

        assert_eq!(decision.decision, PolicyDecisionType::Deny);
        assert!(decision.reason.contains("Confidence") || decision.reason.contains("threshold"));
    }

    #[test]
    fn test_policy_missing_inputs() {
        let policy = Policy::default();
        let mut action = build_test_action("analyze_repository", 0.9);
        action.missing_inputs = Some(vec!["user_input".to_string()]);

        let decision = policy.evaluate(&action);

        assert_eq!(decision.decision, PolicyDecisionType::NeedsInput);
        assert!(decision.reason.contains("Missing inputs"));
    }

    #[test]
    fn test_all_write_actions_denied() {
        let policy = Policy::default();
        let write_actions = vec![
            "create_issue",
            "create_branch",
            "open_draft_pr",
            "post_pr_comment",
            "commit",
            "push",
            "merge",
            "force_push",
            "write_file",
            "delete_file",
        ];

        for action_type in write_actions {
            let action = build_test_action(action_type, 0.9);
            let decision = policy.evaluate(&action);

            assert_eq!(
                decision.decision,
                PolicyDecisionType::Deny,
                "Write action '{}' should be denied",
                action_type
            );
        }
    }

    #[test]
    fn test_confidence_threshold_default() {
        let policy = Policy::default();
        assert_eq!(policy.min_confidence, 0.6);
    }
}
