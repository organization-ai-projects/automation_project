//! projects/products/unstable/neurosymbolic_moe/backend/src/retrieval_engine/tests/simple_retriever.rs
use crate::retrieval_engine::{Chunk, RetrievalQuery, Retriever, SimpleRetriever};

#[test]
fn simple_retriever_search_orders_by_density() {
    let mut retriever = SimpleRetriever::new();
    retriever.add_document(Chunk::new("c1", "rust programming language", "doc1", 0, 25));
    retriever.add_document(Chunk::new("c2", "python scripting", "doc2", 0, 16));
    retriever.add_document(Chunk::new("c3", "rust rust rust", "doc3", 0, 14));

    let query = RetrievalQuery::new("rust");
    let results = retriever.retrieve(&query);
    assert!(results.is_ok());
    let results = results.expect("retrieve should succeed");

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].chunk_id, "c3");
    assert_eq!(results[1].chunk_id, "c1");
}
