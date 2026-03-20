//! projects/products/unstable/neurosymbolic_moe/backend/src/retrieval_engine/tests/retrieval_result.rs
use protocol::ProtocolId;

use crate::retrieval_engine::retrieval_result::RetrievalResult;

#[test]
fn retrieval_result_builder_sets_metadata() {
    let result = RetrievalResult::new(ProtocolId::default(), "content", 0.9, "doc")
        .with_metadata("lang", "rust");
    assert_eq!(result.chunk_id, ProtocolId::default());
    assert_eq!(result.metadata.get("lang"), Some(&"rust".to_string()));
}
