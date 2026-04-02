use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum ProtectError {
    #[error("no expert available for threat: {0}")]
    NoExpertAvailable(String),

    #[error("analysis failed: {0}")]
    AnalysisFailed(String),

    #[error("routing failed: {0}")]
    RoutingFailed(String),

    #[error("aggregation failed: {0}")]
    AggregationFailed(String),

    #[error("expert error: {0}")]
    ExpertError(String),
}
