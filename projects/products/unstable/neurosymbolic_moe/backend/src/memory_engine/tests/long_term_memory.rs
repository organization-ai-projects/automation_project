//! projects/products/unstable/neurosymbolic_moe/backend/src/memory_engine/tests/long_term_memory.rs
use std::collections::HashMap;

use crate::memory_engine::{LongTermMemory, MemoryEntry, MemoryQuery, MemoryStore, MemoryType};

fn make_entry(id: &str, tags: Vec<&str>) -> MemoryEntry {
    MemoryEntry {
        id: id.to_string(),
        content: format!("content-{id}"),
        tags: tags.into_iter().map(String::from).collect(),
        created_at: 1,
        expires_at: None,
        memory_type: MemoryType::Long,
        relevance: 1.0,
        metadata: HashMap::new(),
    }
}

#[test]
fn store_and_retrieve() {
    let mut mem = LongTermMemory::new();
    assert!(mem.store(make_entry("e1", vec!["a"])).is_ok());
    assert!(mem.store(make_entry("e2", vec!["b"])).is_ok());
    assert_eq!(mem.count(), 2);

    let query = MemoryQuery {
        tags: None,
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
fn filter_by_tags() {
    let mut mem = LongTermMemory::new();
    assert!(mem.store(make_entry("e1", vec!["rust"])).is_ok());
    assert!(mem.store(make_entry("e2", vec!["python"])).is_ok());

    let query = MemoryQuery {
        tags: Some(vec!["rust".to_string()]),
        memory_type: None,
        min_relevance: None,
        max_results: 10,
        include_expired: true,
        current_time: Some(0),
    };
    let results = mem.retrieve(&query);
    assert!(results.is_ok());
    let results = results.expect("retrieve should succeed");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "e1");
}
