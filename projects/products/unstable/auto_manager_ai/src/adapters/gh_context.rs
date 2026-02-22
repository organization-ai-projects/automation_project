// projects/products/unstable/auto_manager_ai/src/adapters/gh_context.rs

/// GitHub context information
#[derive(Debug, Clone)]
pub struct GhContext {
    pub available: bool,
    pub status: String,
    pub repo: Option<String>,
    pub default_branch: Option<String>,
    pub open_pr_count: Option<usize>,
    pub degraded: bool,
    pub error_code: Option<String>,
}
