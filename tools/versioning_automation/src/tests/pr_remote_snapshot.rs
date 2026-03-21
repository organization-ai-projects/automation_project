#[test]
fn load_pr_remote_snapshot_returns_error_for_invalid_pr() {
    let result = crate::pr_remote_snapshot::load_pr_remote_snapshot(
        "__invalid_pr__",
        "organization-ai-projects/automation_project",
    );
    assert!(result.is_err());
}
