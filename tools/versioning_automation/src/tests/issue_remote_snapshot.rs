//! tools/versioning_automation/src/tests/issue_remote_snapshot.rs
use crate::issue_remote_snapshot::IssueRemoteSnapshot;

#[test]
fn load_issue_remote_snapshot_returns_error_for_invalid_issue() {
    let result = IssueRemoteSnapshot::load_issue_remote_snapshot(
        "__invalid_issue__",
        Some("organization-ai-projects/automation_project"),
    );
    assert!(result.is_err());
}
