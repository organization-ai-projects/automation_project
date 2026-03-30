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

#[test]
fn load_issue_remote_snapshot_logs_and_handles_valid_issue() {
    let result = IssueRemoteSnapshot::load_issue_remote_snapshot(
        "123", // Use a more realistic issue number
        Some("organization-ai-projects/automation_project"),
    );

    // Assert that the result is a success
    assert!(result.is_ok());

    if let Ok(snapshot) = result {
        println!("[TEST DEBUG] Snapshot loaded: {:?}", snapshot);
    }
}

#[test]
fn load_issue_remote_snapshot_handles_issue_with_hash_prefix() {
    let result = IssueRemoteSnapshot::load_issue_remote_snapshot(
        "#123", // Use an issue number with a hash prefix
        Some("organization-ai-projects/automation_project"),
    );

    // Assert that the result is a success
    assert!(result.is_ok());

    if let Ok(snapshot) = result {
        println!("[TEST DEBUG] Snapshot loaded: {:?}", snapshot);
    }
}
