//projects/products/unstable/neurosymbolic_moe/backend/src/policy_guard/tests/policy_result.rs
use crate::policies_guard::PolicyResult;
use protocol::ProtocolId;
use std::str::FromStr;

#[test]
fn policy_result_fields_round_trip() {
    let policy_id = ProtocolId::from_str("00000000000000000000000000000001")
        .expect("test protocol id should be valid fixed hex");
    let result = PolicyResult {
        policy_id,
        passed: true,
        reason: None,
    };
    assert_eq!(result.policy_id, policy_id);
    assert!(result.passed);
    assert!(result.reason.is_none());
}
