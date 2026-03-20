//! projects/products/unstable/neurosymbolic_moe/backend/src/moe_core/tests/task_id.rs
use crate::moe_core::TaskId;
use protocol::ProtocolId;

fn task_id(_byte: u8) -> TaskId {
    TaskId::from_protocol_id(ProtocolId::default())
}

#[test]
fn task_id_from_protocol_id_is_deterministic() {
    let id = task_id(1);
    assert_eq!(id, task_id(1));
}
