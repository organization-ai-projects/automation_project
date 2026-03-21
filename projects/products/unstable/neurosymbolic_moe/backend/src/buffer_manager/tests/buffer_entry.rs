use crate::buffer_manager::{BufferEntry, BufferType};
use crate::moe_core::TaskId;
use protocol::ProtocolId;
use std::str::FromStr;

fn task_id(byte: u8) -> TaskId {
    TaskId::from_protocol_id(protocol_id(byte))
}

fn protocol_id(byte: u8) -> ProtocolId {
    ProtocolId::from_str(&format!("{:032x}", byte.max(1)))
        .expect("test protocol id should be valid fixed hex")
}

#[test]
fn buffer_entry_builder_methods_set_optional_fields() {
    let entry = BufferEntry::new("k", "v", 5)
        .with_task_id(task_id(1))
        .with_session_id("session-1");

    assert_eq!(entry.key, "k");
    assert_eq!(entry.value, "v");
    assert_eq!(entry.created_at, 5);
    assert_eq!(entry.task_id, Some(task_id(1)));
    assert_eq!(entry.session_id.as_deref(), Some("session-1"));
}

#[test]
fn buffer_type_variants_are_constructible() {
    let working = BufferType::Working;
    let session = BufferType::Session;
    assert!(matches!(working, BufferType::Working));
    assert!(matches!(session, BufferType::Session));
}
