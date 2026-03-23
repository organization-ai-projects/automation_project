//! projects/products/unstable/neurosymbolic_moe/backend/src/memory_engine/tests/short_term_memory.rs
use std::collections::HashMap;

use crate::memory_engine::{MemoryEntry, MemoryQuery, MemoryStore, MemoryType, ShortTermMemory};

fn make_entry(id: &str, created_at: u64, expires_at: Option<u64>) -> MemoryEntry {
    MemoryEntry {
        id: id.to_string(),
        content: format!("content-{id}"),
        tags: vec!["tag1".to_string()],
        created_at,
        expires_at,
        memory_type: MemoryType::Short,
        relevance: 1.0,
        metadata: HashMap::new(),
    }
}

#[test]
fn store_and_retrieve() {
    let mut mem = ShortTermMemory::new(10);
    let r1 = mem.store(make_entry("e1", 1, None));
    let r2 = mem.store(make_entry("e2", 2, None));
    assert!(r1.is_ok());
    assert!(r2.is_ok());
    assert_eq!(mem.count(), 2);

    let query = MemoryQuery {
        tags: Some(vec!["tag1".to_string()]),
        memory_type: None,
        min_relevance: None,
        max_results: 10,
        include_expired: true,
        current_time: Some(0),
    };
    let results = mem.retrieve(&query);
    assert!(results.is_ok());
    assert_eq!(results.expect("retrieve should succeed").len(), 2);
}

#[test]
fn capacity_eviction() {
    let mut mem = ShortTermMemory::new(2);
    assert!(mem.store(make_entry("e1", 1, None)).is_ok());
    assert!(mem.store(make_entry("e2", 2, None)).is_ok());
    assert!(mem.store(make_entry("e3", 3, None)).is_ok());
    assert_eq!(mem.count(), 2);
    assert!(mem.remove("e1").is_none());
}

#[test]
fn expiration() {
    let mut mem = ShortTermMemory::new(10);
    assert!(mem.store(make_entry("e1", 1, Some(100))).is_ok());
    assert!(mem.store(make_entry("e2", 2, Some(200))).is_ok());
    assert!(mem.store(make_entry("e3", 3, None)).is_ok());

    let expired = mem.expire(150);
    assert_eq!(expired, 1);
    assert_eq!(mem.count(), 2);
}
