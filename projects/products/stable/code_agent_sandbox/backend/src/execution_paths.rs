// projects/products/code_agent_sandbox/src/execution_paths.rs
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub(crate) struct ExecutionPaths {
    pub(crate) run_dir: PathBuf,
    pub(crate) work_root: PathBuf,
}

impl ExecutionPaths {
    pub(crate) fn new(run_dir: PathBuf, work_root: PathBuf) -> Self {
        Self { run_dir, work_root }
    }
}
