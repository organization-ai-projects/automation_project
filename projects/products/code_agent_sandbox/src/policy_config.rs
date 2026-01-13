// projects/products/code_agent_sandbox/src/policy_config.rs
use std::default::Default;
use std::path::{Path, PathBuf};

use crate::engine::{EngineConfig, EnginePaths};

#[derive(Clone, Debug)]
pub struct PolicyConfig {
    /// Real repo root (source of truth)
    pub source_repo_root: PathBuf,
    /// Effective working root (real repo in Assist, worktree in Learn)
    pub work_root: PathBuf,
    /// Per-run dir
    pub run_dir: PathBuf,

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
            source_repo_root: paths.repo_root.clone(),
            work_root,
            run_dir: run_dir.to_path_buf(),
            max_read_bytes: config.max_read_bytes,
            max_write_bytes: config.max_write_bytes,
            max_files_per_request: config.max_files_per_request,
            forbid_globs,
            allow_read_globs,
            allow_write_globs,
        }
    }
}

impl Default for PolicyConfig {
    fn default() -> Self {
        PolicyConfig {
            source_repo_root: PathBuf::new(),
            work_root: PathBuf::new(),
            run_dir: PathBuf::new(),
            max_read_bytes: 0,
            max_write_bytes: 0,
            max_files_per_request: 0,
            forbid_globs: Vec::new(),
            allow_read_globs: Vec::new(),
            allow_write_globs: Vec::new(),
        }
    }
}
