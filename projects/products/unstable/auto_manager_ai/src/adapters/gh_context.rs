// projects/products/unstable/auto_manager_ai/src/adapters/gh_context.rs

/// GitHub context information
#[derive(Debug, Clone)]
pub struct GhContext {
    pub available: bool,
    pub info: String,
}
