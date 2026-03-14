use crate::app::{run, run_with};

fn assert_run_and_run_with(args: Vec<String>, expected: i32) {
    let code_run_with = run_with(args.clone());
    let code_run = run(args);
    assert_eq!(code_run_with, expected);
    assert_eq!(code_run, expected);
}

#[test]
fn run_help_returns_zero() {
    let args = vec!["va".to_string(), "help".to_string()];
    assert_run_and_run_with(args, 0);
}

#[test]
fn run_pr_help_returns_zero() {
    let args = vec!["va".to_string(), "pr".to_string(), "--help".to_string()];
    assert_run_and_run_with(args, 0);
}

#[test]
fn run_issue_help_returns_zero() {
    let args = vec!["va".to_string(), "issue".to_string(), "--help".to_string()];
    assert_run_and_run_with(args, 0);
}

#[test]
fn run_pr_without_help_returns_migration_code() {
    let args = vec!["va".to_string(), "pr".to_string(), "--dry-run".to_string()];
    assert_run_and_run_with(args, 2);
}

#[test]
fn run_issue_invalid_args_returns_cli_error_code() {
    let args = vec!["va".to_string(), "issue".to_string(), "close".to_string()];
    assert_run_and_run_with(args, 2);
}

#[test]
fn run_unknown_subcommand_returns_cli_error_code() {
    let args = vec!["va".to_string(), "unknown".to_string()];
    assert_run_and_run_with(args, 2);
}

#[test]
fn run_commit_msg_hook_entrypoint_without_file_returns_cli_error_code() {
    let args = vec!["commit-msg".to_string()];
    assert_run_and_run_with(args, 2);
}

#[test]
fn run_post_checkout_hook_entrypoint_non_branch_returns_zero() {
    let args = vec![
        "post-checkout".to_string(),
        "old".to_string(),
        "new".to_string(),
        "0".to_string(),
    ];
    assert_run_and_run_with(args, 0);
}
