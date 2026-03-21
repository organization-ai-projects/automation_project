#[test]
fn load_issue_remote_snapshot_returns_error_for_invalid_issue() {
    let result = crate::issue_remote_snapshot::load_issue_remote_snapshot(
        "__invalid_issue__",
        Some("organization-ai-projects/automation_project"),
    );
    assert!(result.is_err());
}
