use crate::retrieval_engine::RetrievalResult;

#[test]
fn retrieval_result_builder_sets_metadata() {
    let result = RetrievalResult::new("c1", "content", 0.9, "doc").with_metadata("lang", "rust");
    assert_eq!(result.chunk_id, "c1");
    assert_eq!(result.metadata.get("lang"), Some(&"rust".to_string()));
}
