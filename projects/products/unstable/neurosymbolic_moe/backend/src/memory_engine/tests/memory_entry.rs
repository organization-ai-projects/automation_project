//! projects/products/unstable/neurosymbolic_moe/backend/src/memory_engine/tests/memory_entry.rs
use std::collections::HashMap;

use crate::memory_engine::{MemoryEntry, MemoryQuery, MemoryType};

#[test]
fn memory_entry_and_query_hold_expected_values() {
    let entry = MemoryEntry {
        id: "entry-1".to_string(),
        content: "memo".to_string(),
        tags: vec!["runtime".to_string(), "cache".to_string()],
        created_at: 42,
        expires_at: Some(120),
        memory_type: MemoryType::Medium,
        relevance: 0.75,
        metadata: HashMap::from([("source".to_string(), "unit-test".to_string())]),
    };

    let query = MemoryQuery {
        tags: Some(vec!["runtime".to_string()]),
        memory_type: Some(MemoryType::Medium),
        min_relevance: Some(0.5),
        max_results: 5,
        include_expired: false,
        current_time: Some(50),
    };

    assert_eq!(entry.id, "entry-1");
    assert_eq!(entry.tags.len(), 2);
    assert!(matches!(entry.memory_type, MemoryType::Medium));
    assert_eq!(query.max_results, 5);
    assert_eq!(query.current_time, Some(50));
}
