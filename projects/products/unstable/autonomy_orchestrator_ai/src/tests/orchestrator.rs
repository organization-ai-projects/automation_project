use crate::domain::{
    BinaryInvocationSpec, DeliveryOptions, GateInputs, OrchestratorCheckpoint, OrchestratorConfig,
    PolicyGateStatus, Stage, StageExecutionStatus, TerminalState,
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
        execution_max_iterations: 1,
        reviewer_remediation_max_cycles: 0,
        timeout_ms: 30_000,
        repo_root: PathBuf::from("."),
        planning_context_artifact: None,
        validation_invocations: Vec::new(),
        validation_from_planning_context: false,
        delivery_options: DeliveryOptions::disabled(),
        gate_inputs: GateInputs::passing(),
        checkpoint_path: None,
        cycle_memory_path: None,
        next_actions_path: None,
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
        command: "__missing_binary__".to_string(),
        args: Vec::new(),
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
        command: "__unused__".to_string(),
        args: vec!["x".to_string()],
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
    assert!(
        report
            .blocked_reason_codes
            .contains(&"GATE_POLICY_DENIED_OR_UNKNOWN".to_string())
    );
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
        command: "true".to_string(),
        args: Vec::new(),
        env: Vec::new(),
        timeout_ms: 100,
        expected_artifacts: vec![planner_output_path.display().to_string()],
    });
    let report = Orchestrator::new(config, None).execute();

    assert_eq!(report.terminal_state, Some(TerminalState::Done));
    assert!(
        report
            .stage_executions
            .iter()
            .any(|e| e.command == "planning.planner_output.apply"
                && e.status == StageExecutionStatus::Success)
    );
    assert!(
        report
            .stage_executions
            .iter()
            .any(|e| e.stage == Stage::Validation
                && e.command == "true"
                && e.status == StageExecutionStatus::Success)
    );

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
        command: "true".to_string(),
        args: Vec::new(),
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
