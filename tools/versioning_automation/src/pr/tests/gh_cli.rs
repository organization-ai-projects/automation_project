use crate::gh_cli::{output_trim_cmd, status_cmd};

#[test]
fn gh_output_trim_returns_error_for_invalid_subcommand() {
    let result = output_trim_cmd("__invalid_subcommand__", &["arg"]);
    assert!(result.is_err());
}

#[test]
fn gh_status_returns_non_zero_for_invalid_subcommand() {
    let status = status_cmd("__invalid_subcommand__", &["arg"]);
    assert!(status.is_err());
}
