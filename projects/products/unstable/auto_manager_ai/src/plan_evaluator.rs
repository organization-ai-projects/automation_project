// projects/products/unstable/auto_manager_ai/src/plan_evaluator.rs

use crate::domain::{ActionPlan, Policy, PolicyDecision};

/// Evaluate an action plan against policy
pub fn evaluate_plan(plan: &ActionPlan, policy: &Policy) -> Vec<PolicyDecision> {
    plan.actions
        .iter()
        .map(|action| policy.evaluate(action))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Action, ActionStatus, ActionTarget, Evidence, RiskLevel};

    #[test]
    fn test_evaluate_plan_allows_read_only() {
        let mut plan = ActionPlan::new("Test".to_string());
        plan.add_action(Action {
            id: "test_001".to_string(),
            action_type: "analyze_repository".to_string(),
            status: ActionStatus::Proposed,
            target: ActionTarget::Repo {
                reference: "test/repo".to_string(),
            },
            justification: "Test".to_string(),
            risk_level: RiskLevel::Low,
            required_checks: vec![],
            confidence: 0.9,
            evidence: vec![],
            depends_on: None,
            missing_inputs: None,
            dry_run: None,
        });

        let policy = Policy::default();
        let decisions = evaluate_plan(&plan, &policy);

        assert_eq!(decisions.len(), 1);
        assert_eq!(
            decisions[0].decision,
            crate::domain::PolicyDecisionType::Allow
        );
    }
}
