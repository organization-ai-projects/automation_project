use crate::policy_guard::PolicyResult;

#[test]
fn policy_result_fields_round_trip() {
    let result = PolicyResult {
        policy_id: "p1".to_string(),
        passed: true,
        reason: None,
    };
    assert_eq!(result.policy_id, "p1");
    assert!(result.passed);
    assert!(result.reason.is_none());
}
