use crate::app::run_with;

#[test]
fn run_help_returns_zero() {
    let args = vec!["va".to_string(), "help".to_string()];
    let code = run_with(args);
    assert_eq!(code, 0);
}

#[test]
fn run_pr_help_returns_zero() {
    let args = vec!["va".to_string(), "pr".to_string(), "--help".to_string()];
    let code = run_with(args);
    assert_eq!(code, 0);
}

#[test]
fn run_issue_help_returns_zero() {
    let args = vec!["va".to_string(), "issue".to_string(), "--help".to_string()];
    let code = run_with(args);
    assert_eq!(code, 0);
}

#[test]
fn run_pr_without_help_returns_migration_code() {
    let args = vec!["va".to_string(), "pr".to_string(), "--dry-run".to_string()];
    let code = run_with(args);
    assert_eq!(code, 3);
}

#[test]
fn run_issue_invalid_args_returns_cli_error_code() {
    let args = vec!["va".to_string(), "issue".to_string(), "close".to_string()];
    let code = run_with(args);
    assert_eq!(code, 2);
}

#[test]
fn run_unknown_subcommand_returns_cli_error_code() {
    let args = vec!["va".to_string(), "unknown".to_string()];
    let code = run_with(args);
    assert_eq!(code, 2);
}
