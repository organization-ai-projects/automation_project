// projects/products/core/launcher/src/process.rs
use std::{
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
};

use thiserror::Error;

use crate::Service;

#[derive(Debug, Error)]
pub enum ProcessError {
    #[error("Failed to resolve current working directory")]
    ResolveCwdError,

    #[error("dry-run: spawn skipped")]
    DryRunSkipped,

    #[error("failed to spawn `{0}` from {1}")]
    SpawnError(String, String),
}

/// Resolves the binary path for a given service.
///
/// # Arguments
/// * `workspace_root` - The root directory of the workspace.
/// * `profile_dir` - The profile directory (e.g., debug or release).
/// * `bin` - The name of the binary.
///
/// # Returns
/// The resolved binary path.
pub fn resolve_bin_path(workspace_root: &Path, profile_dir: &Path, bin: &str) -> PathBuf {
    // Unix: target/debug/bin ; Windows: target\debug\bin.exe
    let mut p = workspace_root.join("target").join(profile_dir).join(bin);
    if cfg!(windows) {
        p.set_extension("exe");
    }
    p
}

/// Spawns a service process.
///
/// # Arguments
/// * `svc` - The service to spawn.
/// * `workspace_root` - The root directory of the workspace.
/// * `bin_path` - The path to the binary to execute.
/// * `dry_run` - If true, the service will not actually be started.
///
/// # Returns
/// A handle to the spawned child process.
pub fn spawn_service(
    svc: &Service,
    workspace_root: &Path,
    bin_path: &Path,
    dry_run: bool,
) -> Result<Child, ProcessError> {
    let cwd = svc
        .cwd
        .as_ref()
        .map(|c| workspace_root.join(c))
        .ok_or(ProcessError::ResolveCwdError)?;

    let mut cmd = Command::new(bin_path);
    cmd.args(&svc.args)
        .current_dir(&cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    for kv in &svc.env {
        if let Some((k, v)) = kv.split_once('=') {
            let k: &str = k;
            let v: &str = v;
            cmd.env(k, v);
        }
    }

    println!("▶ spawn {}: {:?}", svc.name, cmd);

    if dry_run {
        return Err(ProcessError::DryRunSkipped);
    }

    cmd.spawn()
        .map_err(|_e| ProcessError::SpawnError(svc.name.clone(), bin_path.display().to_string()))
}
