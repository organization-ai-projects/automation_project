use crate::retrieval_engine::{SimpleRetriever, retriever::Retriever};

#[test]
fn retriever_trait_object_is_wired() {
    let retriever = SimpleRetriever::new();
    let port: &dyn Retriever = &retriever;
    let _ = port;
}
