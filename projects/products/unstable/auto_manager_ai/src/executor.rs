// projects/products/unstable/auto_manager_ai/src/executor.rs

use std::collections::HashSet;

use crate::authz::{AuthzDecision, authorize_action};
use crate::config::Config;
use crate::domain::{ActionPlan, PolicyDecision, policy_decision_type::PolicyDecisionType};
use crate::domain::{RunReport, run_status::RunStatus};

const EXECUTION_ALLOWLIST: &[&str] = &["analyze_repository", "collect_repo_inventory"];

pub fn execute_allowed_actions(
    plan: &ActionPlan,
    decisions: &[PolicyDecision],
    config: &Config,
    report: &mut RunReport,
) {
    let mut seen_action_ids = HashSet::new();

    for action in &plan.actions {
        if !seen_action_ids.insert(action.id.clone()) {
            report.record_execution_blocked(
                action.id.clone(),
                "EXECUTION_IDEMPOTENCY_DUPLICATE",
                "duplicate action id skipped",
            );
            continue;
        }

        let decision = decisions.iter().find(|d| d.action_id == action.id);
        let Some(decision) = decision else {
            report.record_execution_blocked(
                action.id.clone(),
                "EXECUTION_POLICY_DECISION_MISSING",
                "missing policy decision",
            );
            continue;
        };

        if !matches!(decision.decision, PolicyDecisionType::Allow) {
            report.record_execution_blocked(
                action.id.clone(),
                "EXECUTION_POLICY_BLOCKED",
                format!("policy decision={}", decision.decision.as_str()),
            );
            continue;
        }

        let authz = authorize_action(&config.actor, action);
        report.record_authz(
            authz.action_id.clone(),
            authz.reason_code,
            authz.message.clone(),
        );
        if matches!(authz.decision, AuthzDecision::Deny) {
            report.record_execution_blocked(
                action.id.clone(),
                "EXECUTION_AUTHZ_DENIED",
                "execution blocked by authorization",
            );
            continue;
        }

        if !EXECUTION_ALLOWLIST.contains(&action.action_type.as_str()) {
            report.record_execution_blocked(
                action.id.clone(),
                "EXECUTION_ACTION_NOT_ALLOWLISTED",
                format!("action_type={} is not allowlisted", action.action_type),
            );
            continue;
        }

        // Controlled no-op execution path: only low-risk, allowlisted actions are marked executed.
        report.record_execution_success(
            action.id.clone(),
            "EXECUTION_ALLOWLISTED_NOOP",
            format!("executed allowlisted action {}", action.action_type),
        );
    }

    if report.output.actions_executed == 0 && report.status == RunStatus::Success {
        report.record_lifecycle("execution stage completed with zero executed actions");
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        config::Config,
        domain::{
            ActionPlan, PolicyDecision, policy_decision_type::PolicyDecisionType,
            run_report::RunReport,
        },
        executor::execute_allowed_actions,
        tests::test_helpers::build_test_action,
    };
    use std::path::PathBuf;

    #[test]
    fn executes_allowlisted_allowed_action() {
        let mut plan = ActionPlan::new("test".to_string());
        let action = build_test_action("analyze_repository", 0.95);
        let action_id = action.id.clone();
        plan.add_action(action);
        let decision = PolicyDecision {
            action_id: action_id.clone(),
            decision: PolicyDecisionType::Allow,
            reason: "ok".to_string(),
        };

        let config = Config::new(PathBuf::from("."), PathBuf::from("./out"));
        let mut report = RunReport::new("run_x".to_string());
        execute_allowed_actions(&plan, &[decision], &config, &mut report);

        assert_eq!(report.output.actions_executed, 1);
    }

    #[test]
    fn blocks_non_allowlisted_action() {
        let mut plan = ActionPlan::new("test".to_string());
        let action = build_test_action("open_draft_pr", 0.95);
        let action_id = action.id.clone();
        plan.add_action(action);
        let decision = PolicyDecision {
            action_id,
            decision: PolicyDecisionType::Allow,
            reason: "ok".to_string(),
        };

        let config = Config::new(PathBuf::from("."), PathBuf::from("./out"));
        let mut report = RunReport::new("run_y".to_string());
        execute_allowed_actions(&plan, &[decision], &config, &mut report);

        assert_eq!(report.output.actions_executed, 0);
        assert_eq!(report.output.actions_blocked_execution, 1);
    }
}
