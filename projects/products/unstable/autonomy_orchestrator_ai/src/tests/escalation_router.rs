use crate::domain::{
    DecisionContribution, DeliveryOptions, EscalationSeverity, ExecutionPolicy, FinalDecision,
    GateDecision, GateInputs, OrchestratorConfig, PolicyGateStatus, RunReport, Stage,
    StageExecutionRecord, StageExecutionStatus,
};
use crate::escalation_router::route_escalations;
use crate::orchestrator::Orchestrator;
use std::path::PathBuf;

fn base_report(run_id: &str) -> RunReport {
    RunReport::new(run_id.to_string())
}

// ── Trigger-to-severity determinism ──────────────────────────────────────────

#[test]
fn policy_block_trigger_maps_to_sev2() {
    let mut report = base_report("det_run_1");
    report
        .blocked_reason_codes
        .push("GATE_POLICY_DENIED_OR_UNKNOWN".to_string());
    report.gate_decisions.push(GateDecision {
        gate: "policy".to_string(),
        status: "unknown".to_string(),
        passed: false,
        reason_code: Some("GATE_POLICY_DENIED_OR_UNKNOWN".to_string()),
    });

    let cases = route_escalations(&report);

    assert_eq!(cases.len(), 1);
    assert_eq!(cases[0].trigger_code, "ESCALATION_TRIGGER_POLICY_BLOCK");
    assert_eq!(cases[0].severity, EscalationSeverity::Sev2);
}

#[test]
fn critical_tie_trigger_maps_to_sev1() {
    let mut report = base_report("det_run_2");
    report
        .decision_rationale_codes
        .push("DECISION_TIE_FAIL_CLOSED".to_string());

    let cases = route_escalations(&report);

    assert_eq!(cases.len(), 1);
    assert_eq!(cases[0].trigger_code, "ESCALATION_TRIGGER_CRITICAL_TIE");
    assert_eq!(cases[0].severity, EscalationSeverity::Sev1);
}

#[test]
fn cap_exhausted_trigger_via_execution_budget_maps_to_sev3() {
    let mut report = base_report("det_run_3");
    report.stage_executions.push(StageExecutionRecord {
        stage: Stage::Execution,
        idempotency_key: None,
        command: "execution.iteration_budget".to_string(),
        args: vec!["max_iterations_exhausted".to_string(), "5".to_string()],
        env_keys: Vec::new(),
        started_at_unix_secs: 0,
        duration_ms: 0,
        exit_code: None,
        status: StageExecutionStatus::Failed,
        error: Some("Execution iteration budget exhausted".to_string()),
        missing_artifacts: Vec::new(),
        malformed_artifacts: Vec::new(),
    });

    let cases = route_escalations(&report);

    assert_eq!(cases.len(), 1);
    assert_eq!(cases[0].trigger_code, "ESCALATION_TRIGGER_CAP_EXHAUSTED");
    assert_eq!(cases[0].severity, EscalationSeverity::Sev3);
}

#[test]
fn cap_exhausted_trigger_via_remediation_budget_maps_to_sev3() {
    let mut report = base_report("det_run_4");
    report.stage_executions.push(StageExecutionRecord {
        stage: Stage::Validation,
        idempotency_key: None,
        command: "remediation.cycle_budget".to_string(),
        args: vec!["max_cycles_exhausted".to_string(), "3".to_string()],
        env_keys: Vec::new(),
        started_at_unix_secs: 0,
        duration_ms: 0,
        exit_code: None,
        status: StageExecutionStatus::Failed,
        error: Some("Remediation cycle budget exhausted".to_string()),
        missing_artifacts: Vec::new(),
        malformed_artifacts: Vec::new(),
    });

    let cases = route_escalations(&report);

    assert_eq!(cases.len(), 1);
    assert_eq!(cases[0].trigger_code, "ESCALATION_TRIGGER_CAP_EXHAUSTED");
    assert_eq!(cases[0].severity, EscalationSeverity::Sev3);
}

#[test]
fn no_triggers_yields_empty_cases() {
    let report = base_report("det_run_5");
    let cases = route_escalations(&report);
    assert!(cases.is_empty());
}

#[test]
fn multiple_gate_failures_emit_single_policy_block_case() {
    let mut report = base_report("det_run_6");
    for code in [
        "GATE_POLICY_DENIED_OR_UNKNOWN",
        "GATE_CI_NOT_SUCCESS",
        "GATE_REVIEW_NOT_APPROVED",
    ] {
        report.blocked_reason_codes.push(code.to_string());
        report.gate_decisions.push(GateDecision {
            gate: code.to_string(),
            status: "failed".to_string(),
            passed: false,
            reason_code: Some(code.to_string()),
        });
    }

    let cases = route_escalations(&report);

    assert_eq!(cases.len(), 1, "only one POLICY_BLOCK case expected");
    assert_eq!(cases[0].trigger_code, "ESCALATION_TRIGGER_POLICY_BLOCK");
}

#[test]
fn policy_block_case_id_contains_run_id_and_trigger_code() {
    let mut report = base_report("my_run");
    report
        .blocked_reason_codes
        .push("GATE_CI_NOT_SUCCESS".to_string());
    report.gate_decisions.push(GateDecision {
        gate: "ci".to_string(),
        status: "failure".to_string(),
        passed: false,
        reason_code: Some("GATE_CI_NOT_SUCCESS".to_string()),
    });

    let cases = route_escalations(&report);

    assert_eq!(cases[0].id, "my_run:ESCALATION_TRIGGER_POLICY_BLOCK");
}

#[test]
fn context_artifacts_include_failed_gate_names() {
    let mut report = base_report("det_run_7");
    report
        .blocked_reason_codes
        .push("GATE_CI_NOT_SUCCESS".to_string());
    report.gate_decisions.push(GateDecision {
        gate: "ci".to_string(),
        status: "failure".to_string(),
        passed: false,
        reason_code: Some("GATE_CI_NOT_SUCCESS".to_string()),
    });
    report.gate_decisions.push(GateDecision {
        gate: "policy".to_string(),
        status: "allow".to_string(),
        passed: true,
        reason_code: None,
    });

    let cases = route_escalations(&report);

    assert_eq!(cases[0].context_artifacts, vec!["gate:ci"]);
}

#[test]
fn critical_tie_case_has_required_actions() {
    let mut report = base_report("det_run_8");
    report
        .decision_rationale_codes
        .push("DECISION_TIE_FAIL_CLOSED".to_string());

    let cases = route_escalations(&report);

    assert!(!cases[0].required_actions.is_empty());
    assert!(cases[0].required_actions[0].contains("tie"));
}

// ── Integration via Orchestrator ─────────────────────────────────────────────

fn test_config_escalation(run_id: &str) -> OrchestratorConfig {
    OrchestratorConfig {
        run_id: run_id.to_string(),
        simulate_blocked: false,
        planning_invocation: None,
        execution_invocation: None,
        validation_invocation: None,
        execution_policy: ExecutionPolicy {
            execution_max_iterations: 1,
            reviewer_remediation_max_cycles: 0,
        },
        timeout_ms: 30_000,
        repo_root: PathBuf::from("."),
        planning_context_artifact: None,
        validation_invocations: Vec::new(),
        validation_from_planning_context: false,
        delivery_options: DeliveryOptions::disabled(),
        gate_inputs: GateInputs::passing(),
        decision_threshold: 70,
        decision_contributions: vec![DecisionContribution {
            contributor_id: "default".to_string(),
            capability: "governance".to_string(),
            vote: FinalDecision::Proceed,
            confidence: 100,
            weight: 100,
            reason_codes: Vec::new(),
            artifact_refs: Vec::new(),
        }],
        decision_reliability_inputs: Vec::new(),
        decision_require_contributions: false,
        pr_risk_threshold: 40,
        auto_merge_on_eligible: false,
        reviewer_verdicts: Vec::new(),
        checkpoint_path: None,
        cycle_memory_path: None,
        next_actions_path: None,
        previous_run_report_path: None,
        rollout_enabled: false,
        rollback_error_rate_threshold: 0.05,
        rollback_latency_threshold_ms: 5_000,
        memory_path: None,
        memory_max_entries: 500,
        memory_decay_window_runs: 100,
        autofix_enabled: false,
        autofix_bin: None,
        autofix_args: Vec::new(),
        autofix_max_attempts: 3,
        hard_gates_file: None,
        planner_fallback_max_steps: 3,
        risk_tier: None,
        risk_signals: Vec::new(),
        risk_allow_high: false,
    }
}

#[test]
fn normal_path_emits_no_escalation_cases() {
    let report = Orchestrator::new(test_config_escalation("esc_run_1"), None).execute();

    assert!(
        report.escalation_cases.is_empty(),
        "normal done path must not emit escalation cases"
    );
}

#[test]
fn gate_block_path_emits_policy_block_escalation() {
    let mut config = test_config_escalation("esc_run_2");
    config.gate_inputs = GateInputs {
        policy_status: PolicyGateStatus::Unknown,
        ..GateInputs::passing()
    };
    let report = Orchestrator::new(config, None).execute();

    assert_eq!(report.escalation_cases.len(), 1);
    let case = &report.escalation_cases[0];
    assert_eq!(case.trigger_code, "ESCALATION_TRIGGER_POLICY_BLOCK");
    assert_eq!(case.severity, EscalationSeverity::Sev2);
    assert!(!case.required_actions.is_empty());
    assert!(!case.context_artifacts.is_empty());
    assert!(case.id.contains("esc_run_2"));
}

#[test]
fn decision_tie_path_emits_critical_tie_escalation() {
    let mut config = test_config_escalation("esc_run_3");
    // equal-weight Proceed vs Block produces a tie resolved fail-closed
    config.decision_contributions = vec![
        DecisionContribution {
            contributor_id: "a".to_string(),
            capability: "planning".to_string(),
            vote: FinalDecision::Proceed,
            confidence: 80,
            weight: 50,
            reason_codes: Vec::new(),
            artifact_refs: Vec::new(),
        },
        DecisionContribution {
            contributor_id: "b".to_string(),
            capability: "validation".to_string(),
            vote: FinalDecision::Block,
            confidence: 80,
            weight: 50,
            reason_codes: Vec::new(),
            artifact_refs: Vec::new(),
        },
    ];

    let report = Orchestrator::new(config, None).execute();

    assert!(
        report
            .decision_rationale_codes
            .contains(&"DECISION_TIE_FAIL_CLOSED".to_string()),
        "expected DECISION_TIE_FAIL_CLOSED in rationale codes"
    );
    let tie_case = report
        .escalation_cases
        .iter()
        .find(|c| c.trigger_code == "ESCALATION_TRIGGER_CRITICAL_TIE");
    assert!(tie_case.is_some(), "expected CRITICAL_TIE escalation case");
    let case = tie_case.unwrap();
    assert_eq!(case.severity, EscalationSeverity::Sev1);
}
