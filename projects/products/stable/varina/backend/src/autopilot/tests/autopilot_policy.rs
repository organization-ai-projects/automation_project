// projects/products/stable/varina/backend/src/autopilot/tests/autopilot_policy.rs
use crate::autopilot::autopilot_policy::AutopilotPolicy;

#[test]
fn test_default_policy() {
    let policy = AutopilotPolicy::default();
    assert!(policy.protected_branches.contains(&"main".to_string()));
    assert!(policy.protected_branches.contains(&"dev".to_string()));
    assert!(policy.fail_on_unrelated_changes);
    assert!(!policy.allow_push);
}
