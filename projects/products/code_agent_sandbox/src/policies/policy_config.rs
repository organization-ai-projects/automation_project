// projects/products/code_agent_sandbox/src/policies/policy_config.rs
use std::path::{Path, PathBuf};

use crate::engine::{EngineConfig, EnginePaths};
use crate::execution_context::ExecutionContext;

#[derive(Clone, Debug)]
pub struct PolicyConfig {
    pub context: ExecutionContext,

    pub max_read_bytes: usize,
    pub max_write_bytes: usize,
    pub max_files_per_request: usize,

    /// Forbidden paths
    pub forbid_globs: Vec<String>,
    /// Allowed paths for writing
    pub allow_write_globs: Vec<String>,
    /// Allowed paths for reading
    pub allow_read_globs: Vec<String>,
}

impl PolicyConfig {
    pub fn new(
        paths: &EnginePaths,
        run_dir: &Path,
        work_root: PathBuf,
        config: &EngineConfig,
        forbid_globs: Vec<String>,
        allow_read_globs: Vec<String>,
        allow_write_globs: Vec<String>,
    ) -> Self {
        Self {
            context: ExecutionContext::new(paths, run_dir, work_root),
            max_read_bytes: config.max_read_bytes,
            max_write_bytes: config.max_write_bytes,
            max_files_per_request: config.max_files_per_request,
            forbid_globs,
            allow_read_globs,
            allow_write_globs,
        }
    }
}
