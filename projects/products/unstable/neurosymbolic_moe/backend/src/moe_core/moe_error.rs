use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::expert_error::ExpertError;

#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum MoeError {
    #[error("routing failed: {0}")]
    RoutingFailed(String),

    #[error("expert error: {0}")]
    ExpertError(#[from] ExpertError),

    #[error("aggregation failed: {0}")]
    AggregationFailed(String),

    #[error("policy rejected: {0}")]
    PolicyRejected(String),

    #[error("retrieval failed: {0}")]
    RetrievalFailed(String),

    #[error("memory error: {0}")]
    MemoryError(String),

    #[error("dataset error: {0}")]
    DatasetError(String),

    #[error("no expert found: {0}")]
    NoExpertFound(String),
}
