use crate::buffer_manager::BufferManager;
use protocol::ProtocolId;

fn protocol_id(_byte: u8) -> ProtocolId {
    ProtocolId::default()
}

#[test]
fn manager_exposes_working_and_session_buffers() {
    let mut manager = BufferManager::new(3);
    let session_id = protocol_id(1);
    manager.working_mut().put("wk", "wv", None);
    manager.sessions_mut().create_session(&session_id);
    manager.sessions_mut().put(&session_id, "k", "v");

    assert_eq!(manager.working().count(), 1);
    assert_eq!(manager.sessions().session_count(), 1);
}

#[test]
fn clear_all_resets_both_buffers() {
    let mut manager = BufferManager::new(2);
    let session_id = protocol_id(1);
    manager.working_mut().put("k1", "v1", None);
    manager.sessions_mut().create_session(&session_id);
    manager.sessions_mut().put(&session_id, "k", "v");

    manager.clear_all();

    assert_eq!(manager.working().count(), 0);
    assert_eq!(manager.sessions().session_count(), 0);
}
