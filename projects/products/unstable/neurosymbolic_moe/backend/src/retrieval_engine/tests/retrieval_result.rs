//! projects/products/unstable/neurosymbolic_moe/backend/src/retrieval_engine/tests/retrieval_result.rs
use protocol::ProtocolId;
use std::str::FromStr;

use crate::retrieval_engine::retrieval_result::RetrievalResult;

#[test]
fn retrieval_result_builder_sets_metadata() {
    let chunk_id = ProtocolId::from_str("00000000000000000000000000000001")
        .expect("test protocol id should be valid fixed hex");
    let result =
        RetrievalResult::new(chunk_id, "content", 0.9, "doc").with_metadata("lang", "rust");
    assert_eq!(result.chunk_id, chunk_id);
    assert_eq!(result.metadata.get("lang"), Some(&"rust".to_string()));
}
