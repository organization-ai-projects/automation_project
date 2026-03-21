//! projects/products/unstable/neurosymbolic_moe/backend/src/retrieval_engine/tests/retrieval_result.rs
use crate::retrieval_engine::retrieval_result::RetrievalResult;

#[test]
fn retrieval_result_builder_sets_metadata() {
    let chunk_id = crate::tests::helpers::protocol_id(1);
    let result =
        RetrievalResult::new(chunk_id, "content", 0.9, "doc").with_metadata("lang", "rust");
    assert_eq!(result.chunk_id, chunk_id);
    assert_eq!(result.metadata.get("lang"), Some(&"rust".to_string()));
}
