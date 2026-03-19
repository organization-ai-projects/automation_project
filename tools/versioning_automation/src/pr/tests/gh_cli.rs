use crate::pr::gh_cli::{gh_output_trim, gh_status};

#[test]
fn gh_output_trim_returns_error_for_invalid_subcommand() {
    let result = gh_output_trim("__invalid_subcommand__", &["arg"]);
    assert!(result.is_err());
}

#[test]
fn gh_status_returns_non_zero_for_invalid_subcommand() {
    let status = gh_status("__invalid_subcommand__", &["arg"]);
    assert_ne!(status, 0);
}
