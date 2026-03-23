//! projects/products/unstable/neurosymbolic_moe/backend/src/moe_core/tests/task_id.rs
use crate::moe_core::TaskId;
use protocol::ProtocolId;
use std::str::FromStr;

fn task_id(byte: u8) -> TaskId {
    TaskId::from_protocol_id(
        ProtocolId::from_str(&format!("{:032x}", byte.max(1)))
            .expect("test protocol id should be valid fixed hex"),
    )
}

#[test]
fn task_id_from_protocol_id_is_deterministic() {
    let id = task_id(1);
    assert_eq!(id, task_id(1));
}
