// projects/products/code_agent_sandbox/src/engine/engine_paths.rs
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct EnginePaths {
    pub repo_root: PathBuf,
    pub runs_root: PathBuf,
}
