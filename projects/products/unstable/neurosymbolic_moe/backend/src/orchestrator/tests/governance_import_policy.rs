use crate::orchestrator::GovernanceImportPolicy;

#[test]
fn strict_policy_defaults_are_conservative() {
    let policy = GovernanceImportPolicy::strict();
    assert!(!policy.allow_schema_change);
    assert!(!policy.allow_version_regression);
    assert_eq!(policy.max_version_regression, None);
    assert!(!policy.require_policy_match);
}

#[test]
fn default_matches_strict() {
    let default_policy = GovernanceImportPolicy::default();
    let strict_policy = GovernanceImportPolicy::strict();
    assert_eq!(
        default_policy.allow_schema_change,
        strict_policy.allow_schema_change
    );
    assert_eq!(
        default_policy.allow_version_regression,
        strict_policy.allow_version_regression
    );
    assert_eq!(
        default_policy.max_version_regression,
        strict_policy.max_version_regression
    );
    assert_eq!(
        default_policy.require_policy_match,
        strict_policy.require_policy_match
    );
}
