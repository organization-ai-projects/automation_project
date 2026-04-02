//! projects/products/stable/platform_ide/backend/src/offline/tests/offline_policy.rs
use crate::offline::OfflinePolicy;

#[test]
fn default_is_disabled() {
    let policy = OfflinePolicy::default();
    assert!(!policy.is_allowed());
    assert!(policy.require_allowed().is_err());
}

#[test]
fn disabled_helper_matches_default() {
    let policy = OfflinePolicy::disabled();
    assert!(!policy.is_allowed());
}

#[test]
fn allowed_policy_permits() {
    let policy = OfflinePolicy {
        allowed: true,
        notice: Some("Admin has approved offline access.".to_string()),
    };
    assert!(policy.is_allowed());
    assert!(policy.require_allowed().is_ok());
}
