use crate::buffer_manager::BufferManager;

#[test]
fn manager_exposes_working_and_session_buffers() {
    let mut manager = BufferManager::new(3);
    manager.working_mut().put("wk", "wv", None);
    manager.sessions_mut().create_session("s1");
    manager.sessions_mut().put("s1", "k", "v");

    assert_eq!(manager.working().count(), 1);
    assert_eq!(manager.sessions().session_count(), 1);
}

#[test]
fn clear_all_resets_both_buffers() {
    let mut manager = BufferManager::new(2);
    manager.working_mut().put("k1", "v1", None);
    manager.sessions_mut().create_session("s1");
    manager.sessions_mut().put("s1", "k", "v");

    manager.clear_all();

    assert_eq!(manager.working().count(), 0);
    assert_eq!(manager.sessions().session_count(), 0);
}
