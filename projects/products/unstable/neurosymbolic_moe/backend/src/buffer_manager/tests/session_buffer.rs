use crate::buffer_manager::SessionBuffer;

#[test]
fn create_put_get_and_remove_session() {
    let mut buffer = SessionBuffer::new();
    buffer.create_session("s1");
    buffer.put("s1", "k1", "v1");

    let entry = buffer
        .get("s1", "k1")
        .expect("entry should be present in created session");
    assert_eq!(entry.value, "v1");
    assert_eq!(entry.session_id.as_deref(), Some("s1"));
    assert_eq!(buffer.session_count(), 1);
    assert!(buffer.remove_session("s1"));
    assert_eq!(buffer.session_count(), 0);
}

#[test]
fn list_sessions_returns_existing_ids() {
    let mut buffer = SessionBuffer::new();
    buffer.create_session("a");
    buffer.create_session("b");
    let sessions = buffer.list_sessions();
    assert_eq!(sessions.len(), 2);
    assert!(sessions.contains(&"a"));
    assert!(sessions.contains(&"b"));
}
