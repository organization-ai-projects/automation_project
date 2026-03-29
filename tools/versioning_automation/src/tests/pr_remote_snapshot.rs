//! tools/versioning_automation/src/tests/pr_remote_snapshot.rs
use crate::pr_remote_snapshot::PrRemoteSnapshot;

#[test]
fn load_pr_remote_snapshot_returns_error_for_invalid_pr() {
    let result = PrRemoteSnapshot::load_pr_remote_snapshot(
        "__invalid_pr__",
        "organization-ai-projects/automation_project",
    );
    assert!(result.is_err());
}
