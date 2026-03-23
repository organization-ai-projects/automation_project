//! projects/products/unstable/neurosymbolic_moe/backend/src/moe_core/tests/expert_id.rs
use crate::moe_core::ExpertId;
use protocol::ProtocolId;
use std::str::FromStr;

fn expert_id() -> ExpertId {
    ExpertId::from_protocol_id(
        ProtocolId::from_str("00000000000000000000000000000001")
            .expect("test protocol id should be valid fixed hex"),
    )
}

#[test]
fn expert_id_from_protocol_id_is_deterministic() {
    let id = expert_id();
    assert_eq!(id, expert_id());
}
