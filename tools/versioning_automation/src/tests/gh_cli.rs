#[test]
fn output_trim_returns_error_for_invalid_gh_command() {
    let result = crate::gh_cli::output_trim(&["__invalid_subcommand__"]);
    assert!(result.is_err());
}

#[test]
fn output_preserve_returns_error_for_invalid_gh_command() {
    let result = crate::gh_cli::output_preserve(&["__invalid_subcommand__"]);
    assert!(result.is_err());
}
