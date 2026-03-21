use crate::buffer_manager::SessionBuffer;
use protocol::ProtocolId;

fn protocol_id(byte: u8) -> ProtocolId {
    crate::tests::helpers::protocol_id(byte)
}

#[test]
fn create_put_get_and_remove_session() {
    let mut buffer = SessionBuffer::new();
    let session_id = protocol_id(1);
    buffer.create_session(&session_id);
    buffer.put(&session_id, "k1", "v1");

    let entry = buffer
        .get(&session_id, "k1")
        .expect("entry should be present in created session");
    assert_eq!(entry.value, "v1");
    assert_eq!(
        entry.session_id.as_deref(),
        Some(session_id.to_string().as_str())
    );
    assert_eq!(buffer.session_count(), 1);
    assert!(buffer.remove_session(&session_id));
    assert_eq!(buffer.session_count(), 0);
}

#[test]
fn list_sessions_returns_existing_ids() {
    let mut buffer = SessionBuffer::new();
    let session_a = protocol_id(1);
    let session_b = protocol_id(2);
    buffer.create_session(&session_a);
    buffer.create_session(&session_b);
    let sessions = buffer.list_sessions();
    assert_eq!(sessions.len(), 2);
    assert!(sessions.contains(&session_a));
    assert!(sessions.contains(&session_b));
}

#[test]
fn values_ref_returns_sorted_values_without_allocating_owned_strings() {
    let mut buffer = SessionBuffer::new();
    let session_id = protocol_id(1);
    buffer.create_session(&session_id);
    buffer.put(&session_id, "b", "value-b");
    buffer.put(&session_id, "a", "value-a");

    let refs = buffer.values_ref(&session_id);
    assert_eq!(refs, vec!["value-a", "value-b"]);
    let owned = buffer.values(&session_id);
    assert_eq!(owned, vec!["value-a".to_string(), "value-b".to_string()]);
}
