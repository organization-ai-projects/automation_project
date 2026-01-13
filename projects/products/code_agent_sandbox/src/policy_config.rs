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

pub fn build_policy_config(
    paths: &EnginePaths,
    run_dir: &Path,
    work_root: PathBuf,
    config: &EngineConfig,
    forbid_globs: Vec<String>,
    allow_read_globs: Vec<String>,
    allow_write_globs: Vec<String>,
) -> PolicyConfig {
    PolicyConfig {
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
