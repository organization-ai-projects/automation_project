use crate::domain::{
    AdaptivePolicyAction, BinaryInvocationSpec, CommandLineSpec, DecisionContribution,
    DecisionReliabilityInput, DeliveryOptions, ExecutionPolicy, FinalDecision, GateInputs,
    OrchestratorCheckpoint, OrchestratorConfig, PolicyGateStatus, ReviewVerdict, ReviewerVerdict,
    RiskSignal, RiskTier, Stage, StageExecutionStatus, TerminalState,
};
use crate::orchestrator::Orchestrator;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn test_config(run_id: &str) -> OrchestratorConfig {
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
fn executes_all_stages_and_finishes_done() {
    let report = Orchestrator::new(test_config("run_1"), None).execute();

    assert_eq!(report.terminal_state, Some(TerminalState::Done));
    assert_eq!(report.current_stage, Some(Stage::Closure));
    assert_eq!(report.transitions.len(), 5);
    assert_eq!(report.transitions[0].from_stage, None);
    assert_eq!(report.transitions[0].to_stage, Stage::Planning);
    assert!(report.stage_executions.is_empty());
}

#[test]
fn blocked_simulation_stops_before_closure() {
    let mut config = test_config("run_2");
    config.simulate_blocked = true;
    let report = Orchestrator::new(config, None).execute();

    assert_eq!(report.terminal_state, Some(TerminalState::Blocked));
    assert_eq!(report.current_stage, Some(Stage::Validation));
    assert_eq!(report.transitions.len(), 4);
}

#[test]
fn spawn_failure_stops_pipeline_as_failed() {
    let planning_invocation = BinaryInvocationSpec {
        stage: Stage::Planning,
        command_line: CommandLineSpec {
            command: "__missing_binary__".to_string(),
            args: Vec::new(),
        },
        env: Vec::new(),
        timeout_ms: 100,
        expected_artifacts: Vec::new(),
    };

    let mut config = test_config("run_3");
    config.planning_invocation = Some(planning_invocation);
    let report = Orchestrator::new(config, None).execute();

    assert_eq!(report.terminal_state, Some(TerminalState::Failed));
    assert_eq!(report.transitions.len(), 1);
    assert_eq!(report.current_stage, Some(Stage::Planning));
    assert_eq!(report.stage_executions.len(), 1);
}

#[test]
fn resume_skips_completed_invocation_stage() {
    let checkpoint = OrchestratorCheckpoint {
        run_id: "run_4".to_string(),
        completed_stages: vec![Stage::Planning],
        terminal_state: None,
        updated_at_unix_secs: 1,
    };
    let planning_invocation = BinaryInvocationSpec {
        stage: Stage::Planning,
        command_line: CommandLineSpec {
            command: "__unused__".to_string(),
            args: vec!["x".to_string()],
        },
        env: vec![("KEY".to_string(), "VALUE".to_string())],
        timeout_ms: 100,
        expected_artifacts: Vec::new(),
    };

    let mut config = test_config("run_4");
    config.planning_invocation = Some(planning_invocation);
    let report = Orchestrator::new(config, Some(checkpoint)).execute();

    assert_eq!(report.terminal_state, Some(TerminalState::Done));
    assert_eq!(report.stage_executions.len(), 1);
    assert_eq!(
        report.stage_executions[0].status,
        StageExecutionStatus::Skipped
    );
}

#[test]
fn fail_closed_policy_gate_blocks_pipeline_with_reason_code() {
    let mut config = test_config("run_5");
    config.gate_inputs = GateInputs {
        policy_status: PolicyGateStatus::Unknown,
        ..GateInputs::passing()
    };
    let report = Orchestrator::new(config, None).execute();

    assert_eq!(report.terminal_state, Some(TerminalState::Blocked));
    assert!(report
        .blocked_reason_codes
        .contains(&"GATE_POLICY_DENIED_OR_UNKNOWN".to_string()));
    assert_eq!(report.current_stage, Some(Stage::Validation));
}

#[test]
fn planner_output_can_add_validation_commands() {
    let temp_root = std::env::temp_dir().join(format!(
        "autonomy_orchestrator_planner_output_{}_{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&temp_root).expect("create temp directory");
    let planner_output_path = temp_root.join("planner_output.json");
    fs::write(
        &planner_output_path,
        r#"{"planner_output":{"validation_commands":[{"command":"true","args":[]}],"execution_max_iterations":1}}"#,
    )
    .expect("write planner output artifact");

    let mut config = test_config("run_6");
    config.planning_invocation = Some(BinaryInvocationSpec {
        stage: Stage::Planning,
        command_line: CommandLineSpec {
            command: "true".to_string(),
            args: Vec::new(),
        },
        env: Vec::new(),
        timeout_ms: 100,
        expected_artifacts: vec![planner_output_path.display().to_string()],
    });
    let report = Orchestrator::new(config, None).execute();

    assert_eq!(report.terminal_state, Some(TerminalState::Done));
    assert!(report
        .stage_executions
        .iter()
        .any(|e| e.command == "planning.planner_output.apply"
            && e.status == StageExecutionStatus::Success));
    assert!(report
        .stage_executions
        .iter()
        .any(|e| e.stage == Stage::Validation
            && e.command == "true"
            && e.status == StageExecutionStatus::Success));

    fs::remove_dir_all(&temp_root).ok();
}

#[test]
fn planner_output_with_zero_execution_budget_fails_closed() {
    let temp_root = std::env::temp_dir().join(format!(
        "autonomy_orchestrator_planner_output_invalid_{}_{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&temp_root).expect("create temp directory");
    let planner_output_path = temp_root.join("planner_output_invalid.json");
    fs::write(
        &planner_output_path,
        r#"{"planner_output":{"execution_max_iterations":0}}"#,
    )
    .expect("write invalid planner output artifact");

    let mut config = test_config("run_7");
    config.planning_invocation = Some(BinaryInvocationSpec {
        stage: Stage::Planning,
        command_line: CommandLineSpec {
            command: "true".to_string(),
            args: Vec::new(),
        },
        env: Vec::new(),
        timeout_ms: 100,
        expected_artifacts: vec![planner_output_path.display().to_string()],
    });
    let report = Orchestrator::new(config, None).execute();

    assert_eq!(report.terminal_state, Some(TerminalState::Failed));
    assert!(report.stage_executions.iter().any(|e| {
        e.command == "planning.planner_output.apply"
            && e.status == StageExecutionStatus::Failed
            && e.error
                .as_deref()
                .is_some_and(|err| err.contains("execution_max_iterations must be >= 1"))
    }));

    fs::remove_dir_all(&temp_root).ok();
}

#[test]
fn decision_contributions_can_block_closure_with_reason_code() {
    let mut config = test_config("run_8");
    config.decision_contributions = vec![DecisionContribution {
        contributor_id: "reviewer".to_string(),
        capability: "validation".to_string(),
        vote: FinalDecision::Escalate,
        confidence: 95,
        weight: 80,
        reason_codes: vec!["REVIEWER_ESCALATION".to_string()],
        artifact_refs: Vec::new(),
    }];
    let report = Orchestrator::new(config, None).execute();

    assert_eq!(report.terminal_state, Some(TerminalState::Blocked));
    assert_eq!(report.final_decision, Some(FinalDecision::Escalate));
    assert!(report
        .blocked_reason_codes
        .contains(&"DECISION_ESCALATED".to_string()));
}

#[test]
fn decision_require_contributions_fails_closed_when_empty() {
    let mut config = test_config("run_9");
    config.decision_contributions = Vec::new();
    config.decision_require_contributions = true;
    let report = Orchestrator::new(config, None).execute();

    assert_eq!(report.terminal_state, Some(TerminalState::Blocked));
    assert_eq!(report.final_decision, Some(FinalDecision::Block));
    assert!(report
        .blocked_reason_codes
        .contains(&"DECISION_NO_CONTRIBUTIONS".to_string()));
}

#[test]
fn adaptive_policy_can_increase_execution_budget_once() {
    let mut config = test_config("run_10");
    config.execution_invocation = Some(BinaryInvocationSpec {
        stage: Stage::Execution,
        command_line: CommandLineSpec {
            command: "false".to_string(),
            args: Vec::new(),
        },
        env: Vec::new(),
        timeout_ms: 100,
        expected_artifacts: Vec::new(),
    });
    config.execution_policy.execution_max_iterations = 1;

    let report = Orchestrator::new(config, None).execute();
    assert_eq!(report.terminal_state, Some(TerminalState::Failed));
    assert!(report
        .adaptive_policy_decisions
        .iter()
        .any(|decision| decision.action == AdaptivePolicyAction::IncreaseExecutionBudget));
    let execution_failures = report
        .stage_executions
        .iter()
        .filter(|e| e.stage == Stage::Execution && e.status == StageExecutionStatus::Failed)
        .count();
    assert!(
        execution_failures >= 2,
        "expected at least two execution failures after adaptive budget increase, got {execution_failures}"
    );
}

#[test]
fn adaptive_policy_does_not_increase_execution_budget_when_cap_is_reached() {
    let mut config = test_config("run_11");
    config.execution_invocation = Some(BinaryInvocationSpec {
        stage: Stage::Execution,
        command_line: CommandLineSpec {
            command: "false".to_string(),
            args: Vec::new(),
        },
        env: Vec::new(),
        timeout_ms: 100,
        expected_artifacts: Vec::new(),
    });
    // Matches AdaptivePolicyConfig::default() cap.
    config.execution_policy.execution_max_iterations = 5;

    let report = Orchestrator::new(config, None).execute();
    assert_eq!(report.terminal_state, Some(TerminalState::Failed));
    assert!(report
        .adaptive_policy_decisions
        .iter()
        .all(|decision| decision.action != AdaptivePolicyAction::IncreaseExecutionBudget));
    let execution_failures = report
        .stage_executions
        .iter()
        .filter(|e| e.stage == Stage::Execution && e.status == StageExecutionStatus::Failed)
        .count();
    assert_eq!(
        execution_failures, 6,
        "expected exactly 5 failed attempts + 1 budget exhaustion record"
    );
}

#[test]
fn reliability_factors_and_updates_are_persisted_in_run_report() {
    let mut config = test_config("run_12");
    config.decision_contributions = vec![
        DecisionContribution {
            contributor_id: "planner".to_string(),
            capability: "planning".to_string(),
            vote: FinalDecision::Proceed,
            confidence: 80,
            weight: 70,
            reason_codes: Vec::new(),
            artifact_refs: Vec::new(),
        },
        DecisionContribution {
            contributor_id: "reviewer".to_string(),
            capability: "validation".to_string(),
            vote: FinalDecision::Block,
            confidence: 60,
            weight: 70,
            reason_codes: Vec::new(),
            artifact_refs: Vec::new(),
        },
    ];
    config.decision_reliability_inputs = vec![
        DecisionReliabilityInput {
            contributor_id: "planner".to_string(),
            capability: "planning".to_string(),
            score: 90,
        },
        DecisionReliabilityInput {
            contributor_id: "reviewer".to_string(),
            capability: "validation".to_string(),
            score: 10,
        },
    ];

    let report = Orchestrator::new(config, None).execute();
    assert_eq!(report.terminal_state, Some(TerminalState::Done));
    assert_eq!(report.final_decision, Some(FinalDecision::Proceed));
    assert_eq!(report.decision_reliability_factors.len(), 2);
    assert_eq!(report.decision_reliability_updates.len(), 2);
    assert!(report
        .decision_rationale_codes
        .contains(&"DECISION_RELIABILITY_WEIGHTED".to_string()));
}

#[test]
fn review_ensemble_result_and_verdicts_persisted_in_run_report() {
    let mut config = test_config("run_13");
    config.reviewer_verdicts = vec![
        ReviewerVerdict {
            reviewer_id: "r1".to_string(),
            specialty: "correctness".to_string(),
            verdict: ReviewVerdict::Approve,
            confidence: 90,
            weight: 80,
            reason_codes: Vec::new(),
        },
        ReviewerVerdict {
            reviewer_id: "r2".to_string(),
            specialty: "security".to_string(),
            verdict: ReviewVerdict::Approve,
            confidence: 85,
            weight: 80,
            reason_codes: Vec::new(),
        },
        ReviewerVerdict {
            reviewer_id: "r3".to_string(),
            specialty: "maintainability".to_string(),
            verdict: ReviewVerdict::Approve,
            confidence: 80,
            weight: 60,
            reason_codes: Vec::new(),
        },
    ];
}

#[test]
fn planner_path_record_is_persisted_in_run_report() {
    let temp_root = std::env::temp_dir().join(format!(
        "autonomy_orchestrator_path_record_{}_{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&temp_root).expect("create temp directory");
    let planner_output_path = temp_root.join("planner_output_with_graph.json");
    fs::write(
        &planner_output_path,
        r#"{
            "planner_output": {
                "planner_nodes": [
                    {"id": "start", "action": "plan_start"},
                    {"id": "end", "action": "plan_end"}
                ],
                "planner_edges": [
                    {"from": "start", "to": "end", "condition_code": ""}
                ]
            }
        }"#,
    )
    .expect("write planner output artifact");

    let mut config = test_config("run_path_record");
    config.planning_invocation = Some(BinaryInvocationSpec {
        stage: Stage::Planning,
        command_line: CommandLineSpec {
            command: "true".to_string(),
            args: Vec::new(),
        },
        env: Vec::new(),
        timeout_ms: 100,
        expected_artifacts: vec![planner_output_path.display().to_string()],
    });

    let report = Orchestrator::new(config, None).execute();

    assert_eq!(report.terminal_state, Some(TerminalState::Done));
    assert_eq!(report.reviewer_verdicts.len(), 3);
    let ensemble = report
        .review_ensemble_result
        .expect("ensemble result must be present");
    assert!(ensemble.passed);
    assert!(ensemble.reason_codes.is_empty());
}

#[test]
fn review_ensemble_security_rejection_blocks_run() {
    let mut config = test_config("run_14");
    config.reviewer_verdicts = vec![
        ReviewerVerdict {
            reviewer_id: "r1".to_string(),
            specialty: "correctness".to_string(),
            verdict: ReviewVerdict::Approve,
            confidence: 100,
            weight: 100,
            reason_codes: Vec::new(),
        },
        ReviewerVerdict {
            reviewer_id: "r2".to_string(),
            specialty: "security".to_string(),
            verdict: ReviewVerdict::Reject,
            confidence: 90,
            weight: 1,
            reason_codes: Vec::new(),
        },
    ];

    let report = Orchestrator::new(config, None).execute();

    assert_eq!(report.terminal_state, Some(TerminalState::Blocked));
    assert!(report
        .blocked_reason_codes
        .contains(&"REVIEW_ENSEMBLE_SECURITY_REJECTION".to_string()));
    let ensemble = report
        .review_ensemble_result
        .expect("ensemble result must be present");
    assert!(!ensemble.passed);
}

#[test]
fn review_ensemble_tie_blocks_run_fail_closed() {
    let mut config = test_config("run_15");
    config.reviewer_verdicts = vec![
        ReviewerVerdict {
            reviewer_id: "r1".to_string(),
            specialty: "correctness".to_string(),
            verdict: ReviewVerdict::Approve,
            confidence: 80,
            weight: 50,
            reason_codes: Vec::new(),
        },
        ReviewerVerdict {
            reviewer_id: "r2".to_string(),
            specialty: "maintainability".to_string(),
            verdict: ReviewVerdict::Reject,
            confidence: 80,
            weight: 50,
            reason_codes: Vec::new(),
        },
    ];
    let path_record = report
        .planner_path_record
        .expect("planner_path_record must be set");
    assert_eq!(path_record.selected_path, vec!["start", "end"]);
    assert_eq!(path_record.fallback_steps_used, 0);
    assert!(path_record.reason_codes.is_empty());

    fs::remove_dir_all(&temp_root).ok();
}

#[test]
fn invalid_planner_graph_fails_closed_with_reason_code() {
    let temp_root = std::env::temp_dir().join(format!(
        "autonomy_orchestrator_invalid_graph_{}_{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&temp_root).expect("create temp directory");
    let planner_output_path = temp_root.join("planner_output_invalid_graph.json");
    fs::write(
        &planner_output_path,
        r#"{
            "planner_output": {
                "planner_nodes": [
                    {"id": "a", "action": "step_a"},
                    {"id": "a", "action": "step_a_dup"}
                ],
                "planner_edges": []
            }
        }"#,
    )
    .expect("write planner output artifact");

    let mut config = test_config("run_invalid_graph");
    config.planning_invocation = Some(BinaryInvocationSpec {
        stage: Stage::Planning,
        command_line: CommandLineSpec {
            command: "true".to_string(),
            args: Vec::new(),
        },
        env: Vec::new(),
        timeout_ms: 100,
        expected_artifacts: vec![planner_output_path.display().to_string()],
    });

    let report = Orchestrator::new(config, None).execute();

    assert_eq!(report.terminal_state, Some(TerminalState::Failed));
    assert!(report
        .blocked_reason_codes
        .contains(&"PLANNER_GRAPH_INVALID".to_string()));

    fs::remove_dir_all(&temp_root).ok();
}

#[test]
fn planner_fallback_budget_exhaustion_fails_closed() {
    let temp_root = std::env::temp_dir().join(format!(
        "autonomy_orchestrator_fallback_budget_{}_{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&temp_root).expect("create temp directory");
    let planner_output_path = temp_root.join("planner_output_fallback.json");
    // Graph: a →(ON_FAIL) b →(ON_FAIL) c →(ON_FAIL) d; max fallback steps = 2
    fs::write(
        &planner_output_path,
        r#"{
            "planner_output": {
                "planner_nodes": [
                    {"id": "a", "action": "step_a"},
                    {"id": "b", "action": "step_b"},
                    {"id": "c", "action": "step_c"},
                    {"id": "d", "action": "step_d"}
                ],
                "planner_edges": [
                    {"from": "a", "to": "b", "condition_code": "ON_FAIL"},
                    {"from": "b", "to": "c", "condition_code": "ON_FAIL"},
                    {"from": "c", "to": "d", "condition_code": "ON_FAIL"}
                ]
            }
        }"#,
    )
    .expect("write planner output artifact");

    let mut config = test_config("run_fallback_budget");
    config.planner_fallback_max_steps = 2;
    config.planning_invocation = Some(BinaryInvocationSpec {
        stage: Stage::Planning,
        command_line: CommandLineSpec {
            command: "true".to_string(),
            args: Vec::new(),
        },
        env: Vec::new(),
        timeout_ms: 100,
        expected_artifacts: vec![planner_output_path.display().to_string()],
    });

    let report = Orchestrator::new(config, None).execute();

    assert_eq!(report.terminal_state, Some(TerminalState::Failed));
    let path_record = report
        .planner_path_record
        .expect("planner_path_record must be set");
    assert_eq!(path_record.fallback_steps_used, 2);
    assert!(path_record
        .reason_codes
        .contains(&"PLANNER_FALLBACK_BUDGET_EXHAUSTED".to_string()));
    assert!(report
        .stage_executions
        .iter()
        .any(|e| e.command == "planning.planner_v2.select_path"
            && e.status == StageExecutionStatus::Failed));

    fs::remove_dir_all(&temp_root).ok();
}

#[test]
fn planner_fallback_within_budget_succeeds() {
    let temp_root = std::env::temp_dir().join(format!(
        "autonomy_orchestrator_fallback_ok_{}_{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&temp_root).expect("create temp directory");
    let planner_output_path = temp_root.join("planner_output_fallback_ok.json");
    // Graph: a →(ON_FAIL) b →(primary) c; 1 fallback step within budget of 3
    fs::write(
        &planner_output_path,
        r#"{
            "planner_output": {
                "planner_nodes": [
                    {"id": "a", "action": "step_a"},
                    {"id": "b", "action": "step_b"},
                    {"id": "c", "action": "step_c"}
                ],
                "planner_edges": [
                    {"from": "a", "to": "b", "condition_code": "ON_FAIL"},
                    {"from": "b", "to": "c", "condition_code": ""}
                ]
            }
        }"#,
    )
    .expect("write planner output artifact");

    let mut config = test_config("run_fallback_ok");
    config.planner_fallback_max_steps = 3;
    config.planning_invocation = Some(BinaryInvocationSpec {
        stage: Stage::Planning,
        command_line: CommandLineSpec {
            command: "true".to_string(),
            args: Vec::new(),
        },
        env: Vec::new(),
        timeout_ms: 100,
        expected_artifacts: vec![planner_output_path.display().to_string()],
    });

    let report = Orchestrator::new(config, None).execute();

    assert_eq!(report.terminal_state, Some(TerminalState::Done));
    let path_record = report
        .planner_path_record
        .expect("planner_path_record must be set");
    assert_eq!(path_record.selected_path, vec!["a", "b", "c"]);
    assert_eq!(path_record.fallback_steps_used, 1);
    assert!(path_record
        .reason_codes
        .contains(&"PLANNER_FALLBACK_STEP_APPLIED".to_string()));

    fs::remove_dir_all(&temp_root).ok();
}

#[test]
fn run_report_includes_risk_tier_and_signals() {
    let mut config = test_config("run_risk_1");
    config.risk_tier = Some(RiskTier::Medium);
    config.risk_signals = vec![RiskSignal {
        code: "DELIVERY_WRITE_ENABLED".to_string(),
        source: "delivery_options".to_string(),
        value: "feature/x".to_string(),
    }];
    config.risk_allow_high = false;

    let report = Orchestrator::new(config, None).execute();

    assert_eq!(report.risk_tier, Some(RiskTier::Medium));
    assert_eq!(report.risk_signals.len(), 1);
    assert_eq!(report.risk_signals[0].code, "DELIVERY_WRITE_ENABLED");
    // medium risk does not block
    assert_eq!(report.terminal_state, Some(TerminalState::Done));
}

#[test]
fn high_risk_is_blocked_by_default() {
    let mut config = test_config("run_risk_2");
    config.risk_tier = Some(RiskTier::High);
    config.risk_signals = vec![RiskSignal {
        code: "DELIVERY_TO_PROTECTED_BRANCH".to_string(),
        source: "delivery_options".to_string(),
        value: "main".to_string(),
    }];
    config.risk_allow_high = false;

    let report = Orchestrator::new(config, None).execute();

    assert_eq!(report.terminal_state, Some(TerminalState::Blocked));
    assert!(report
        .blocked_reason_codes
        .contains(&"REVIEW_ENSEMBLE_TIE_FAIL_CLOSED".to_string()));
    assert_eq!(report.risk_tier, Some(RiskTier::High));
    assert!(report
        .blocked_reason_codes
        .contains(&"RISK_TIER_HIGH_BLOCKED".to_string()));
}

#[test]
fn high_risk_proceeds_when_explicitly_allowed() {
    let mut config = test_config("run_risk_3");
    config.risk_tier = Some(RiskTier::High);
    config.risk_signals = vec![RiskSignal {
        code: "DELIVERY_TO_PROTECTED_BRANCH".to_string(),
        source: "delivery_options".to_string(),
        value: "main".to_string(),
    }];
    config.risk_allow_high = true;

    let report = Orchestrator::new(config, None).execute();

    // high risk allowed, so execution reaches Done
    assert_eq!(report.terminal_state, Some(TerminalState::Done));
    assert_eq!(report.risk_tier, Some(RiskTier::High));
    assert!(!report
        .blocked_reason_codes
        .contains(&"RISK_TIER_HIGH_BLOCKED".to_string()));
}

#[test]
fn low_risk_run_report_has_none_tier_when_not_classified() {
    let config = test_config("run_risk_4");
    // risk_tier defaults to None

    let report = Orchestrator::new(config, None).execute();

    assert_eq!(report.risk_tier, None);
    assert!(report.risk_signals.is_empty());
    assert_eq!(report.terminal_state, Some(TerminalState::Done));
}
