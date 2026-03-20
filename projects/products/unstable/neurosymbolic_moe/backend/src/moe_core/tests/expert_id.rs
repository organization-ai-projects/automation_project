//! projects/products/unstable/neurosymbolic_moe/backend/src/moe_core/tests/expert_id.rs
use crate::moe_core::ExpertId;
use protocol::ProtocolId;

fn expert_id() -> ExpertId {
    ExpertId::from_protocol_id(ProtocolId::default())
}

#[test]
fn expert_id_from_protocol_id_is_deterministic() {
    let id = expert_id();
    assert_eq!(id, expert_id());
}
