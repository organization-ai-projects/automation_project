use crate::buffer_manager::WorkingBuffer;

#[test]
fn put_and_get() {
    let mut buffer = WorkingBuffer::new(10);
    buffer.put("k1", "v1", None);
    let entry = buffer
        .get("k1")
        .expect("entry should exist after insertion");
    assert_eq!(entry.value, "v1");
    assert_eq!(buffer.count(), 1);
}

#[test]
fn capacity_eviction() {
    let mut buffer = WorkingBuffer::new(2);
    buffer.put("k1", "v1", None);
    buffer.put("k2", "v2", None);
    buffer.put("k3", "v3", None);
    assert_eq!(buffer.count(), 2);
    assert!(buffer.get("k1").is_none());
    assert!(buffer.get("k2").is_some());
    assert!(buffer.get("k3").is_some());
}
