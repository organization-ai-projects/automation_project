//! projects/products/unstable/neurosymbolic_moe/backend/src/retrieval_engine/tests/simple_retriever.rs
use protocol::ProtocolId;

use crate::retrieval_engine::{Chunk, Retriever, SimpleRetriever, retrieval_query::RetrievalQuery};

#[test]
fn simple_retriever_search_orders_by_density() {
    let mut retriever = SimpleRetriever::new();
    retriever.add_document(Chunk::new(
        ProtocolId::default(),
        "rust programming language",
        "doc1",
        0,
        25,
    ));
    retriever.add_document(Chunk::new(
        ProtocolId::default(),
        "python scripting",
        "doc2",
        0,
        16,
    ));
    retriever.add_document(Chunk::new(
        ProtocolId::default(),
        "rust rust rust",
        "doc3",
        0,
        14,
    ));

    let query = RetrievalQuery::new("rust");
    let results = retriever.retrieve(&query);
    assert!(results.is_ok());
    let results = results.expect("retrieve should succeed");

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].content, "rust rust rust");
    assert_eq!(results[1].content, "rust programming language");
}
