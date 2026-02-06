// projects/products/unstable/auto_manager_ai/src/domain/policy.rs

use super::action::Action;
use super::policy_decision::PolicyDecision;
use super::policy_decision_type::PolicyDecisionType;

/// Policy rules for evaluating actions
#[derive(Debug, Clone)]
pub struct Policy {
    pub min_confidence: f64,
    pub allow_repo_writes: bool,
    pub allow_github_writes: bool,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            min_confidence: 0.6,
            allow_repo_writes: false,  // V0: no repo writes
            allow_github_writes: false, // V0: no GitHub writes
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
        if let Some(missing) = &action.missing_inputs {
            if !missing.is_empty() {
                return PolicyDecision {
                    action_id: action.id.clone(),
                    decision: PolicyDecisionType::NeedsInput,
                    reason: format!("Missing inputs: {}", missing.join(", ")),
                };
            }
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
