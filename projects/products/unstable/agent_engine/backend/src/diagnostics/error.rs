// projects/products/unstable/agent_engine/backend/src/diagnostics/error.rs
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] common_json::JsonError),
    #[error("usage error: {0}")]
    Usage(String),
    #[error("step execution failed: {0}")]
    StepFailed(String),
}
