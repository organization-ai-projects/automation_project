// projects/products/code_agent_sandbox/src/execution_context.rs
use std::path::{Path, PathBuf};

use crate::{execution_paths::ExecutionPaths, sandbox_engine};

#[derive(Clone, Debug)]
pub(crate) struct ExecutionContext {
    pub(crate) source_repo_root: PathBuf,
    pub(crate) paths: ExecutionPaths,
}

impl ExecutionContext {
    pub(crate) fn new(
        paths: &sandbox_engine::EnginePaths,
        run_dir: &Path,
        work_root: PathBuf,
    ) -> Self {
        Self {
            source_repo_root: paths.repo_root.clone(),
            paths: ExecutionPaths::new(run_dir.to_path_buf(), work_root),
        }
    }
}
