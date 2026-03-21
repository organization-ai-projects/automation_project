//! projects/products/unstable/neurosymbolic_moe/backend/src/moe_core/tests/task_id.rs
use crate::moe_core::TaskId;

fn task_id(byte: u8) -> TaskId {
    crate::tests::helpers::task_id(byte)
}

#[test]
fn task_id_from_protocol_id_is_deterministic() {
    let id = task_id(1);
    assert_eq!(id, task_id(1));
}
