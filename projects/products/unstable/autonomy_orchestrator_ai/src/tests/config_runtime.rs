use crate::cli_command::Cli;
use crate::config_runtime::{
    derive_config_io_plan, first_non_binary_config_path, is_binary_config_path,
    validate_orchestrator_config,
};
use crate::configs::{ConfigLoadMode, ConfigSaveMode};
use crate::domain::{DeliveryOptions, ExecutionPolicy, GateInputs, OrchestratorConfig};
use clap::Parser;
use std::path::PathBuf;

fn parse_run_args(argv: &[&str]) -> crate::run_args::RunArgs {
    Cli::parse_from(argv).run
}

fn base_config() -> OrchestratorConfig {
    OrchestratorConfig {
        run_id: "run_cfg_test".to_string(),
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
        decision_contributions: Vec::new(),
        decision_reliability_inputs: Vec::new(),
        decision_require_contributions: false,
        pr_risk_threshold: 40,
        auto_merge_on_eligible: false,
        checkpoint_path: None,
        cycle_memory_path: None,
        next_actions_path: None,
        previous_run_report_path: None,
    }
}

#[test]
fn derive_config_io_plan_sets_single_load_mode_and_multiple_saves() {
    let args = parse_run_args(&[
        "autonomy_orchestrator_ai",
        "--config-load-bin",
        "/tmp/in.bin",
        "--config-save-bin",
        "/tmp/out.bin",
        "--config-save-json",
        "/tmp/out.json",
    ]);

    let plan = derive_config_io_plan(&args).expect("plan should be valid");
    assert!(matches!(
        plan.load.as_ref(),
        Some(ConfigLoadMode::Bin(path)) if path == &PathBuf::from("/tmp/in.bin")
    ));
    assert_eq!(plan.saves.len(), 2);
    assert!(plan.saves.iter().any(
        |mode| matches!(mode, ConfigSaveMode::Bin(path) if path == &PathBuf::from("/tmp/out.bin"))
    ));
    assert!(plan.saves.iter().any(|mode| matches!(
        mode,
        ConfigSaveMode::Json(path) if path == &PathBuf::from("/tmp/out.json")
    )));
}

#[test]
fn derive_config_io_plan_rejects_multiple_load_modes() {
    let args = parse_run_args(&[
        "autonomy_orchestrator_ai",
        "--config-load-bin",
        "/tmp/in.bin",
        "--config-load-json",
        "/tmp/in.json",
    ]);

    let err = derive_config_io_plan(&args).expect_err("expected load mode conflict");
    assert!(err.contains("Only one config load mode is allowed"));
}

#[test]
fn derive_config_io_plan_rejects_auto_save_mixed_with_explicit_save_mode() {
    let args = parse_run_args(&[
        "autonomy_orchestrator_ai",
        "--config-save",
        "/tmp/out",
        "--config-save-bin",
        "/tmp/out.bin",
    ]);

    let err = derive_config_io_plan(&args).expect_err("expected save mode conflict");
    assert!(err.contains("When --config-save is used"));
}

#[test]
fn first_non_binary_config_path_returns_first_non_binary_candidate() {
    let args = parse_run_args(&[
        "autonomy_orchestrator_ai",
        "--config-load-ron",
        "/tmp/in.ron",
        "--config-save-bin",
        "/tmp/out.bin",
    ]);
    let plan = derive_config_io_plan(&args).expect("plan should be valid");

    let first = first_non_binary_config_path(&plan).expect("should detect non-binary path");
    assert_eq!(first, PathBuf::from("/tmp/in.ron").as_path());
}

#[test]
fn is_binary_config_path_accepts_bin_and_extensionless_paths() {
    assert!(is_binary_config_path(
        PathBuf::from("/tmp/conf.bin").as_path()
    ));
    assert!(is_binary_config_path(
        PathBuf::from("/tmp/conf.BIN").as_path()
    ));
    assert!(is_binary_config_path(PathBuf::from("/tmp/conf").as_path()));
    assert!(!is_binary_config_path(
        PathBuf::from("/tmp/conf.json").as_path()
    ));
}

#[test]
fn validate_orchestrator_config_reports_all_main_invariants() {
    let mut config = base_config();
    config.timeout_ms = 0;
    config.execution_policy.execution_max_iterations = 0;
    config.validation_from_planning_context = true;
    config.delivery_options.pr_enabled = true;
    config.delivery_options.enabled = false;

    let diagnostics = validate_orchestrator_config(&config);
    assert_eq!(diagnostics.len(), 4);
    assert!(
        diagnostics
            .iter()
            .any(|d| d.contains("timeout_ms must be > 0"))
    );
    assert!(
        diagnostics
            .iter()
            .any(|d| d.contains("execution_max_iterations must be >= 1"))
    );
    assert!(
        diagnostics
            .iter()
            .any(|d| d.contains("validation_from_planning_context=true requires"))
    );
    assert!(
        diagnostics
            .iter()
            .any(|d| d.contains("delivery_options.pr_enabled=true requires"))
    );
}
