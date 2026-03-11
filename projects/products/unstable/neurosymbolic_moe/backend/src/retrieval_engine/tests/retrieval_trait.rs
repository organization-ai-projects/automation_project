use crate::moe_core::{ExpertId, TaskId};
use crate::retrieval_engine::{RetrievalQuery, RetrievalResult};

#[test]
fn retrieval_query_builder_sets_fields() {
    let query = RetrievalQuery::new("rust")
        .with_task_id(TaskId::new("t1"))
        .with_expert_id(ExpertId::new("e1"))
        .with_max_results(3)
        .with_min_relevance(0.4)
        .with_filter("domain", "systems");

    assert_eq!(query.query, "rust");
    assert_eq!(query.max_results, 3);
    assert_eq!(query.min_relevance, 0.4);
    assert_eq!(query.filters.get("domain"), Some(&"systems".to_string()));
}

#[test]
fn retrieval_result_builder_sets_metadata() {
    let result = RetrievalResult::new("c1", "content", 0.9, "doc").with_metadata("lang", "rust");
    assert_eq!(result.chunk_id, "c1");
    assert_eq!(result.metadata.get("lang"), Some(&"rust".to_string()));
}
