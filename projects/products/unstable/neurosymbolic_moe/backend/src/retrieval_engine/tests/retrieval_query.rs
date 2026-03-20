//! projects/products/unstable/neurosymbolic_moe/backend/src/retrieval_engine/tests/retrieval_query.rs
use crate::{
    moe_core::{ExpertId, TaskId},
    retrieval_engine::retrieval_query::RetrievalQuery,
};

#[test]
fn retrieval_query_builder_sets_fields() {
    let query = RetrievalQuery::new("rust")
        .with_task_id(TaskId::new())
        .with_expert_id(ExpertId::new())
        .with_max_results(3)
        .with_min_relevance(0.4)
        .with_filter("domain", "systems");

    assert_eq!(query.query, "rust");
    assert_eq!(query.max_results, 3);
    assert_eq!(query.min_relevance, 0.4);
    assert_eq!(query.filters.get("domain"), Some(&"systems".to_string()));
}
