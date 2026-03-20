#[test]
fn run_help_returns_zero() {
    let args = vec!["help".to_string()];
    let code = super::super::execute::run(&args);
    assert_eq!(code, 0);
}

#[test]
fn run_unknown_subcommand_returns_two() {
    let args = vec!["unknown-subcommand".to_string()];
    let code = super::super::execute::run(&args);
    assert_eq!(code, 2);
}

#[test]
fn resolve_scope_from_file_path_uses_crate_root_for_tool_sources() {
    let root = super::super::execute::repo_root().expect("repo root");
    let scope = super::super::execute::resolve_scope_from_file_path(
        &root,
        "tools/versioning_automation/src/issues/execute.rs",
    );
    assert_eq!(scope.as_deref(), Some("tools/versioning_automation"));
}

#[test]
fn resolve_scope_from_file_path_uses_crate_root_for_library_sources() {
    let root = super::super::execute::repo_root().expect("repo root");
    let scope = super::super::execute::resolve_scope_from_file_path(
        &root,
        "projects/libraries/core/contracts/protocol/src/protocol_id.rs",
    );
    assert_eq!(
        scope.as_deref(),
        Some("projects/libraries/core/contracts/protocol")
    );
}
