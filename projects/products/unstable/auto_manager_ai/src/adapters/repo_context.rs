// projects/products/unstable/auto_manager_ai/src/adapters/repo_context.rs

use std::path::PathBuf;

/// Context about the repository
#[derive(Debug, Clone)]
pub struct RepoContext {
    pub root: PathBuf,
    pub tracked_files: Vec<String>,
    pub mediated_by_engine: bool,
}
