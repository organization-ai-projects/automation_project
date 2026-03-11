use crate::moe_core::MoeError;

use super::retrieval_query::RetrievalQuery;
use super::retrieval_result::RetrievalResult;

pub trait Retriever {
    fn retrieve(&self, query: &RetrievalQuery) -> Result<Vec<RetrievalResult>, MoeError>;
}
