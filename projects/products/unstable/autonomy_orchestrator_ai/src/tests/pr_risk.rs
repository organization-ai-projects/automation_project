// projects/products/unstable/autonomy_orchestrator_ai/src/tests/pr_risk.rs
use crate::domain::{
    GateDecision, RunReport, Stage, StageExecutionRecord, StageExecutionStatus, TerminalState,
};
use crate::pr_risk::compute_pr_risk;

fn base_report() -> RunReport {
    RunReport::new("test_run".to_string())
}

#[test]
fn zero_score_for_clean_done_run() {
    let mut report = base_report();
    report.terminal_state = Some(TerminalState::Done);
    report.gate_decisions = vec![
        GateDecision {
            gate: "policy".to_string(),
            status: "allow".to_string(),
            passed: true,
            reason_code: None,
        },
        GateDecision {
            gate: "ci".to_string(),
            status: "success".to_string(),
            passed: true,
            reason_code: None,
        },
        GateDecision {
            gate: "review".to_string(),
            status: "approved".to_string(),
            passed: true,
            reason_code: None,
        },
    ];
    report.decision_confidence = Some(95);

    let breakdown = compute_pr_risk(&report, 40);

    assert_eq!(breakdown.total_score, 0);
    assert!(breakdown.eligible_for_auto_merge);
    assert_eq!(breakdown.factors.len(), 5);
    assert!(breakdown.factors.iter().all(|f| f.score == 0));
}

#[test]
fn all_gates_failing_produces_max_gate_score() {
    let mut report = base_report();
    report.terminal_state = Some(TerminalState::Blocked);
    report.gate_decisions = vec![
        GateDecision {
            gate: "policy".to_string(),
            status: "unknown".to_string(),
            passed: false,
            reason_code: Some("GATE_POLICY_DENIED_OR_UNKNOWN".to_string()),
        },
        GateDecision {
            gate: "ci".to_string(),
            status: "failure".to_string(),
            passed: false,
            reason_code: Some("GATE_CI_NOT_SUCCESS".to_string()),
        },
        GateDecision {
            gate: "review".to_string(),
            status: "missing".to_string(),
            passed: false,
            reason_code: Some("GATE_REVIEW_NOT_APPROVED".to_string()),
        },
    ];
    report.blocked_reason_codes = vec![
        "GATE_POLICY_DENIED_OR_UNKNOWN".to_string(),
        "GATE_CI_NOT_SUCCESS".to_string(),
        "GATE_REVIEW_NOT_APPROVED".to_string(),
    ];

    let breakdown = compute_pr_risk(&report, 40);

    // gate_quality=36, risk_tier=8, hard_gate_status=12 → at least 56
    assert!(breakdown.total_score >= 56);
    assert!(!breakdown.eligible_for_auto_merge);

    let gate_factor = breakdown
        .factors
        .iter()
        .find(|f| f.name == "gate_quality")
        .unwrap();
    assert_eq!(gate_factor.score, 36);
}

#[test]
fn low_decision_confidence_increases_score() {
    let mut report = base_report();
    report.terminal_state = Some(TerminalState::Done);
    report.decision_confidence = Some(30);

    let breakdown_low = compute_pr_risk(&report, 40);
    let conf_factor_low = breakdown_low
        .factors
        .iter()
        .find(|f| f.name == "decision_confidence")
        .unwrap();
    assert_eq!(conf_factor_low.score, 24);

    report.decision_confidence = Some(90);
    let breakdown_high = compute_pr_risk(&report, 40);
    let conf_factor_high = breakdown_high
        .factors
        .iter()
        .find(|f| f.name == "decision_confidence")
        .unwrap();
    assert_eq!(conf_factor_high.score, 0);

    assert!(breakdown_high.total_score < breakdown_low.total_score);
}

#[test]
fn validation_failures_increase_test_stability_score() {
    let mut report = base_report();
    report.terminal_state = Some(TerminalState::Done);

    fn failed_exec(stage: Stage) -> StageExecutionRecord {
        StageExecutionRecord {
            stage,
            idempotency_key: None,
            command: "test_cmd".to_string(),
            args: Vec::new(),
            env_keys: Vec::new(),
            started_at_unix_secs: 0,
            duration_ms: 0,
            exit_code: Some(1),
            status: StageExecutionStatus::Failed,
            error: None,
            missing_artifacts: Vec::new(),
            malformed_artifacts: Vec::new(),
        }
    }

    // No failures → score 0
    let breakdown_no_fail = compute_pr_risk(&report, 40);
    let stability_no_fail = breakdown_no_fail
        .factors
        .iter()
        .find(|f| f.name == "test_stability")
        .unwrap();
    assert_eq!(stability_no_fail.score, 0);

    // 1 validation failure → score 6
    report.stage_executions.push(failed_exec(Stage::Validation));
    let breakdown_one = compute_pr_risk(&report, 40);
    let stability_one = breakdown_one
        .factors
        .iter()
        .find(|f| f.name == "test_stability")
        .unwrap();
    assert_eq!(stability_one.score, 6);

    // Non-validation failure should not affect test_stability
    report.stage_executions.push(failed_exec(Stage::Execution));
    let breakdown_exec = compute_pr_risk(&report, 40);
    let stability_exec = breakdown_exec
        .factors
        .iter()
        .find(|f| f.name == "test_stability")
        .unwrap();
    assert_eq!(stability_exec.score, 6);

    // 3 validation failures → score 12
    report.stage_executions.push(failed_exec(Stage::Validation));
    report.stage_executions.push(failed_exec(Stage::Validation));
    let breakdown_many = compute_pr_risk(&report, 40);
    let stability_many = breakdown_many
        .factors
        .iter()
        .find(|f| f.name == "test_stability")
        .unwrap();
    assert_eq!(stability_many.score, 12);
}

#[test]
fn eligibility_boundary_at_threshold_is_inclusive() {
    let mut report = base_report();
    report.terminal_state = Some(TerminalState::Done);
    // Manually construct a case that produces exactly score 40
    // risk_tier=0 (Done), confidence=None (0), gate_quality=0, hard_gate=0
    // Add 4 validation failures (3+ → 12) — still <= 40? No, 12 <= 40, eligible.
    // Let's set it up: risk_tier=16 (None terminal), conf=24 → total=40
    report.terminal_state = None;
    report.decision_confidence = Some(30); // → confidence_score = 24

    let breakdown = compute_pr_risk(&report, 40);
    // risk_tier=16, confidence=24, rest=0 → total=40
    assert_eq!(breakdown.total_score, 40);
    assert!(breakdown.eligible_for_auto_merge, "score == threshold should be eligible");

    // One above threshold: add 1 non-passing gate (12 more → total=52)
    report.gate_decisions = vec![GateDecision {
        gate: "ci".to_string(),
        status: "failure".to_string(),
        passed: false,
        reason_code: Some("GATE_CI_NOT_SUCCESS".to_string()),
    }];
    let breakdown_above = compute_pr_risk(&report, 40);
    assert!(!breakdown_above.eligible_for_auto_merge, "score above threshold should not be eligible");
}

#[test]
fn determinism_same_inputs_produce_same_output() {
    let mut report = base_report();
    report.terminal_state = Some(TerminalState::Blocked);
    report.decision_confidence = Some(55);
    report.gate_decisions = vec![GateDecision {
        gate: "ci".to_string(),
        status: "pending".to_string(),
        passed: false,
        reason_code: Some("GATE_CI_NOT_SUCCESS".to_string()),
    }];
    report.blocked_reason_codes = vec!["GATE_CI_NOT_SUCCESS".to_string()];

    let b1 = compute_pr_risk(&report, 40);
    let b2 = compute_pr_risk(&report, 40);

    assert_eq!(b1, b2);
}
