//! projects/products/unstable/neurosymbolic_moe/backend/src/retrieval_engine/retriever.rs
use crate::moe_core::MoeError;

use super::retrieval_query::RetrievalQuery;
use super::retrieval_result::RetrievalResult;

pub trait Retriever: Send + Sync {
    fn retrieve(&self, query: &RetrievalQuery) -> Result<Vec<RetrievalResult>, MoeError>;
}
