// projects/products/unstable/auto_manager_ai/src/adapters/ci_context.rs

/// CI context information
#[allow(dead_code)] // Reserved for future use in planners and reports
#[derive(Debug, Clone)]
pub struct CiContext {
    pub available: bool,
    pub info: String,
}
