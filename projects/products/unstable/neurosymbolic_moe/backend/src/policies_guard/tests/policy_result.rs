//projects/products/unstable/neurosymbolic_moe/backend/src/policy_guard/tests/policy_result.rs
use crate::policies_guard::PolicyResult;

#[test]
fn policy_result_fields_round_trip() {
    let policy_id = crate::tests::helpers::protocol_id(1);
    let result = PolicyResult {
        policy_id,
        passed: true,
        reason: None,
    };
    assert_eq!(result.policy_id, policy_id);
    assert!(result.passed);
    assert!(result.reason.is_none());
}
