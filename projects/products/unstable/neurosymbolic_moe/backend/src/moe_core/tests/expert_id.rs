//! projects/products/unstable/neurosymbolic_moe/backend/src/moe_core/tests/expert_id.rs
use crate::moe_core::ExpertId;

fn expert_id() -> ExpertId {
    crate::tests::helpers::expert_id(1)
}

#[test]
fn expert_id_from_protocol_id_is_deterministic() {
    let id = expert_id();
    assert_eq!(id, expert_id());
}
