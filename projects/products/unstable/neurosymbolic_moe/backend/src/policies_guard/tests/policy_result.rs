//projects/products/unstable/neurosymbolic_moe/backend/src/policy_guard/tests/policy_result.rs
use protocol::ProtocolId;

use crate::policies_guard::PolicyResult;

#[test]
fn policy_result_fields_round_trip() {
    let result = PolicyResult {
        policy_id: ProtocolId::default(),
        passed: true,
        reason: None,
    };
    assert_eq!(result.policy_id, ProtocolId::default());
    assert!(result.passed);
    assert!(result.reason.is_none());
}
