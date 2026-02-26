use crate::cli_command::Cli;
use clap::Parser;

#[test]
fn cli_run_args_have_expected_defaults() {
    let cli = Cli::parse_from(["autonomy_orchestrator_ai"]);

    assert!(cli.command.is_none());
    assert_eq!(cli.run.output_dir.to_string_lossy(), "./out");
    assert!(!cli.run.simulate_blocked);
    assert!(!cli.run.resume);
    assert!(!cli.run.verbose);
    assert_eq!(cli.run.timeout_ms, 30_000);
    assert_eq!(cli.run.execution_max_iterations, 1);
    assert_eq!(cli.run.reviewer_remediation_max_cycles, 0);
}

#[test]
fn cli_accepts_repeated_manager_executor_and_reviewer_values() {
    let cli = Cli::parse_from([
        "autonomy_orchestrator_ai",
        "--manager-bin",
        "manager",
        "--manager-arg=--plan",
        "--manager-env",
        "A=1",
        "--executor-bin",
        "executor",
        "--executor-arg=--apply",
        "--executor-env",
        "B=2",
        "--reviewer-bin",
        "reviewer",
        "--reviewer-arg=--gate",
        "--reviewer-env",
        "C=3",
    ]);

    assert_eq!(cli.run.manager_bin.as_deref(), Some("manager"));
    assert_eq!(cli.run.manager_args, vec!["--plan".to_string()]);
    assert_eq!(
        cli.run.manager_env,
        vec![("A".to_string(), "1".to_string())]
    );

    assert_eq!(cli.run.executor_bin.as_deref(), Some("executor"));
    assert_eq!(cli.run.executor_args, vec!["--apply".to_string()]);
    assert_eq!(
        cli.run.executor_env,
        vec![("B".to_string(), "2".to_string())]
    );

    assert_eq!(cli.run.reviewer_bin.as_deref(), Some("reviewer"));
    assert_eq!(cli.run.reviewer_args, vec!["--gate".to_string()]);
    assert_eq!(
        cli.run.reviewer_env,
        vec![("C".to_string(), "3".to_string())]
    );
}

#[test]
fn cli_rejects_invalid_env_pair_for_manager_env() {
    let err = Cli::try_parse_from(["autonomy_orchestrator_ai", "--manager-env", "BROKEN"])
        .expect_err("expected parse error");

    let rendered = err.to_string();
    assert!(rendered.contains("Invalid env pair 'BROKEN', expected KEY=VALUE"));
}
