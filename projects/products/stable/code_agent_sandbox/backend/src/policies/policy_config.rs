// projects/products/code_agent_sandbox/src/policies/policy_config.rs
use std::path::{Path, PathBuf};

use crate::execution_context::ExecutionContext;
use crate::sandbox_engine::{EngineConfig, EnginePaths};

#[derive(Clone, Debug)]
pub(crate) struct PolicyConfig {
    pub(crate) context: ExecutionContext,

    pub(crate) max_read_bytes: usize,
    pub(crate) max_write_bytes: usize,
    pub(crate) max_files_per_request: usize,
    /// Forbidden paths
    pub(crate) forbid_globs: Vec<String>,
    /// Allowed paths for writing
    pub(crate) allow_write_globs: Vec<String>,
    /// Allowed paths for reading
    pub(crate) allow_read_globs: Vec<String>,
}

impl PolicyConfig {
    pub(crate) fn new(
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
