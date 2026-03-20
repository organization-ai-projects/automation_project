//! tools/versioning_automation/src/automation/hook_checks/tests/pre_commit.rs
use crate::automation::{commands::PreCommitCheckOptions, hook_checks::pre_commit};

#[test]
fn run_pre_commit_check_returns_ok_when_skip_enabled() {
    let result = pre_commit::run_pre_commit_check_with_skip(PreCommitCheckOptions, true);
    assert!(result.is_ok());
}

#[test]
fn pre_commit_exits_early_when_no_staged_files() {
    assert!(pre_commit::should_exit_pre_commit_early(&[]));
}
