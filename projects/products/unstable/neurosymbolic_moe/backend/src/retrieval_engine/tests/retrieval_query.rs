use crate::moe_core::{ExpertId, TaskId};
use crate::retrieval_engine::RetrievalQuery;

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
