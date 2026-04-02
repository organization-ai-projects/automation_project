//! projects/products/unstable/neurosymbolic_moe/backend/src/memory_engine/tests/memory_store.rs
use std::collections::HashMap;

use crate::memory_engine::{MemoryEntry, MemoryQuery, MemoryStore, MemoryType, ShortTermMemory};

fn insert_and_count<S: MemoryStore>(store: &mut S, entry: MemoryEntry) -> usize {
    let stored = store.store(entry);
    assert!(stored.is_ok());
    store.count()
}

#[test]
fn memory_store_trait_is_usable_via_generic_bound() {
    let mut store = ShortTermMemory::new(4);
    let entry = MemoryEntry {
        id: "generic-1".to_string(),
        content: "generic".to_string(),
        tags: vec!["trait".to_string()],
        created_at: 1,
        expires_at: None,
        memory_type: MemoryType::Short,
        relevance: 1.0,
        metadata: HashMap::new(),
    };

    assert_eq!(insert_and_count(&mut store, entry), 1);

    let query = MemoryQuery {
        tags: Some(vec!["trait".to_string()]),
        memory_type: Some(MemoryType::Short),
        min_relevance: Some(0.1),
        max_results: 10,
        include_expired: true,
        current_time: Some(0),
    };

    let retrieved = store.retrieve(&query);
    assert!(retrieved.is_ok());
    assert_eq!(retrieved.expect("retrieve should succeed").len(), 1);
}
