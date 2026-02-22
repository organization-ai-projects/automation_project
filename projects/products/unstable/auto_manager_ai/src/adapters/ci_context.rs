// projects/products/unstable/auto_manager_ai/src/adapters/ci_context.rs

/// CI context information
#[derive(Debug, Clone)]
pub struct CiContext {
    pub available: bool,
    pub status: String,
    pub provider: String,
    pub run_id: Option<String>,
    pub workflow: Option<String>,
    pub job: Option<String>,
    pub degraded: bool,
    pub error_code: Option<String>,
}
