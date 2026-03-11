use crate::buffer_manager::{BufferEntry, BufferType};
use crate::moe_core::TaskId;

#[test]
fn buffer_entry_builder_methods_set_optional_fields() {
    let entry = BufferEntry::new("k", "v", 5)
        .with_task_id(TaskId::new("task-1"))
        .with_session_id("session-1");

    assert_eq!(entry.key, "k");
    assert_eq!(entry.value, "v");
    assert_eq!(entry.created_at, 5);
    assert_eq!(entry.task_id.as_ref().map(TaskId::as_str), Some("task-1"));
    assert_eq!(entry.session_id.as_deref(), Some("session-1"));
}

#[test]
fn buffer_type_variants_are_constructible() {
    let working = BufferType::Working;
    let session = BufferType::Session;
    assert!(matches!(working, BufferType::Working));
    assert!(matches!(session, BufferType::Session));
}
