//! tools/versioning_automation/src/issues/commands/tests/open_snapshots_options.rs
use crate::issues::commands::open_snapshots_options::OpenSnapshotsOptions;

#[test]
fn test_run_open_snapshots_with_repo() {
    let options = OpenSnapshotsOptions {
        repo: Some("test_repo".to_string()),
        limit: 10,
    };
    let result = options.run_open_snapshots();
    assert_eq!(result, 0);
}

#[test]
fn test_run_open_snapshots_without_repo() {
    let options = OpenSnapshotsOptions {
        repo: None,
        limit: 5,
    };
    let result = options.run_open_snapshots();
    assert_eq!(result, 0);
}
