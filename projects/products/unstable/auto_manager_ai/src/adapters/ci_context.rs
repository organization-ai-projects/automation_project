// projects/products/unstable/auto_manager_ai/src/adapters/ci_context.rs

/// CI context information
#[derive(Debug, Clone)]
pub struct CiContext {
    pub available: bool,
    pub info: String,
}
