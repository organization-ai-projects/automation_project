//! tools/versioning_automation/src/automation/hook_checks/tests/pre_push.rs
use crate::automation::commands::PrePushCheckOptions;

#[test]
fn run_pre_push_check_returns_ok_when_skip_enabled() {
    let result = super::super::pre_push::run_pre_push_check_with_skip(PrePushCheckOptions, true);
    assert!(result.is_ok());
}
