// projects/products/unstable/autonomy_orchestrator_ai/src/escalation_router.rs
use crate::domain::{EscalationCase, EscalationSeverity, RunReport, StageExecutionStatus};

/// Deterministically route escalation cases from the run report.
///
/// Trigger-to-severity mapping:
/// - `ESCALATION_TRIGGER_POLICY_BLOCK` (hard-gate violation / no allow flag) → Sev2
/// - `ESCALATION_TRIGGER_CRITICAL_TIE` (critical tie fail-closed)           → Sev1
/// - `ESCALATION_TRIGGER_CAP_EXHAUSTED` (adaptive cap / rollback exhaustion) → Sev3
pub fn route_escalations(report: &RunReport) -> Vec<EscalationCase> {
    let mut cases = Vec::new();

    // Trigger: hard-gate violation or high-risk run without allow flag.
    // Source: blocked_reason_codes containing gate denial codes.
    if report.blocked_reason_codes.iter().any(|c| {
        matches!(
            c.as_str(),
            "GATE_POLICY_DENIED_OR_UNKNOWN"
                | "GATE_CI_NOT_SUCCESS"
                | "GATE_REVIEW_NOT_APPROVED"
        )
    }) {
        let context_artifacts = report
            .gate_decisions
            .iter()
            .filter(|g| !g.passed)
            .map(|g| format!("gate:{}", g.gate))
            .collect();
        cases.push(EscalationCase {
            id: format!("{}:ESCALATION_TRIGGER_POLICY_BLOCK", report.run_id),
            trigger_code: "ESCALATION_TRIGGER_POLICY_BLOCK".to_string(),
            severity: EscalationSeverity::Sev2,
            required_actions: vec![
                "Resolve policy gate before autonomous rerun".to_string(),
            ],
            context_artifacts,
        });
    }

    // Trigger: critical tie fail-closed.
    // Source: decision_rationale_codes or blocked_reason_codes contains DECISION_TIE_FAIL_CLOSED.
    if report
        .decision_rationale_codes
        .contains(&"DECISION_TIE_FAIL_CLOSED".to_string())
        || report
            .blocked_reason_codes
            .contains(&"DECISION_TIE_FAIL_CLOSED".to_string())
    {
        let context_artifacts = report
            .decision_contributions
            .iter()
            .map(|c| format!("contributor:{}", c.contributor_id))
            .collect();
        cases.push(EscalationCase {
            id: format!("{}:ESCALATION_TRIGGER_CRITICAL_TIE", report.run_id),
            trigger_code: "ESCALATION_TRIGGER_CRITICAL_TIE".to_string(),
            severity: EscalationSeverity::Sev1,
            required_actions: vec![
                "Resolve conflicting agent votes to remove fail-closed tie".to_string(),
            ],
            context_artifacts,
        });
    }

    // Trigger: adaptive cap exhaustion (execution iteration budget) or
    // rollback loop exhaustion (remediation cycle budget).
    // Source: stage_executions records emitted when the respective budget is fully spent.
    let cap_exhausted = report.stage_executions.iter().any(|e| {
        matches!(
            e.command.as_str(),
            "execution.iteration_budget" | "remediation.cycle_budget"
        ) && e.status == StageExecutionStatus::Failed
    });
    if cap_exhausted {
        cases.push(EscalationCase {
            id: format!("{}:ESCALATION_TRIGGER_CAP_EXHAUSTED", report.run_id),
            trigger_code: "ESCALATION_TRIGGER_CAP_EXHAUSTED".to_string(),
            severity: EscalationSeverity::Sev3,
            required_actions: vec![
                "Inspect exhausted iteration budget and adjust policy caps".to_string(),
            ],
            context_artifacts: vec![format!("run:{}", report.run_id)],
        });
    }

    cases
}
