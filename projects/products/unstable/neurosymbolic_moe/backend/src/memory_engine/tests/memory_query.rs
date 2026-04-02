use crate::memory_engine::{MemoryQuery, MemoryType};

#[test]
fn memory_query_fields_round_trip() {
    let query = MemoryQuery {
        tags: Some(vec!["tag-a".to_string()]),
        memory_type: Some(MemoryType::Medium),
        min_relevance: Some(0.4),
        max_results: 5,
        include_expired: false,
        current_time: Some(100),
    };
    assert_eq!(query.tags.as_ref().map(Vec::len), Some(1));
    assert!(matches!(query.memory_type, Some(MemoryType::Medium)));
    assert_eq!(query.min_relevance, Some(0.4));
    assert_eq!(query.max_results, 5);
    assert!(!query.include_expired);
    assert_eq!(query.current_time, Some(100));
}
