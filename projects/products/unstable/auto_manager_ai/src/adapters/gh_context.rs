// projects/products/unstable/auto_manager_ai/src/adapters/gh_context.rs

/// GitHub context information
#[allow(dead_code)] // Reserved for future use in planners and reports
#[derive(Debug, Clone)]
pub struct GhContext {
    pub available: bool,
    pub info: String,
}
