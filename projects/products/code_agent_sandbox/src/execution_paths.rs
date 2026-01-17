// projects/products/code_agent_sandbox/src/execution_paths.rs
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct ExecutionPaths {
    pub run_dir: PathBuf,
    pub work_root: PathBuf,
}

impl ExecutionPaths {
    pub fn new(run_dir: PathBuf, work_root: PathBuf) -> Self {
        Self { run_dir, work_root }
    }
}
