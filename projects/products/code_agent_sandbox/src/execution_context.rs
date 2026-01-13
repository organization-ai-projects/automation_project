// projects/products/code_agent_sandbox/src/execution_context.rs
use std::path::{Path, PathBuf};

use crate::{engine, execution_paths::ExecutionPaths};

#[derive(Clone, Debug)]
pub struct ExecutionContext {
    pub source_repo_root: PathBuf,
    pub paths: ExecutionPaths,
}

impl ExecutionContext {
    pub fn new(paths: &engine::EnginePaths, run_dir: &Path, work_root: PathBuf) -> Self {
        Self {
            source_repo_root: paths.repo_root.clone(),
            paths: ExecutionPaths::new(run_dir.to_path_buf(), work_root),
        }
    }
}
