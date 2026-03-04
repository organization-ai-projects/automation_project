// projects/products/code_agent_sandbox/src/engine/engine_paths.rs
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub(crate) struct EnginePaths {
    pub(crate) repo_root: PathBuf,
    pub(crate) runs_root: PathBuf,
}
