use crate::retrieval_engine::{Retriever, SimpleRetriever};

#[test]
fn retriever_trait_object_is_wired() {
    let retriever = SimpleRetriever::new();
    let port: &dyn Retriever = &retriever;
    let _ = port;
}
