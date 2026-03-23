#[test]
fn output_trim_reads_repo_state() {
    let result = crate::git_cli::output_trim(&["rev-parse", "--is-inside-work-tree"]);
    assert!(matches!(result.as_deref(), Ok("true") | Ok("false")));
}

#[test]
fn output_trim_returns_error_for_invalid_git_command() {
    let result = crate::git_cli::output_trim(&["__invalid_subcommand__"]);
    assert!(result.is_err());
}
