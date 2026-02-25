// projects/products/stable/core/launcher/src/process.rs
use std::{
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
};

use anyhow::{Context, Result, anyhow};

use crate::service::Service;

/// Resolves the binary path for a given service.
///
/// # Arguments
/// * `workspace_root` - The root directory of the workspace.
/// * `profile_dir` - The profile directory (e.g., debug or release).
/// * `bin` - The name of the binary.
///
/// # Returns
/// The resolved binary path.
pub(crate) fn resolve_bin_path(workspace_root: &Path, profile_dir: &Path, bin: &str) -> PathBuf {
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
pub(crate) fn spawn_service(
    svc: &Service,
    workspace_root: &Path,
    bin_path: &Path,
    dry_run: bool,
) -> Result<Child> {
    let cwd = svc
        .cwd
        .as_ref()
        .map(|c| workspace_root.join(c))
        .unwrap_or_else(|| workspace_root.to_path_buf());

    let mut cmd = Command::new(bin_path);
    cmd.args(&svc.args)
        .current_dir(&cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut has_watcher_config = false;
    for kv in &svc.env {
        if let Some((k, v)) = kv.split_once('=') {
            if k == "WATCHER_CONFIG" {
                has_watcher_config = true;
            }
            cmd.env(k, v);
        }
    }
    configure_watcher_env(svc, workspace_root, has_watcher_config, &mut cmd);

    println!("▶ spawn {}: {:?}", svc.name, cmd);

    if dry_run {
        // dummy child is impossible; caller won't use it because dry_run returns earlier.
        return Err(anyhow!("dry-run: spawn skipped"));
    }

    cmd.spawn()
        .with_context(|| format!("failed to spawn `{}` from {}", svc.name, bin_path.display()))
}

fn configure_watcher_env(
    svc: &Service,
    workspace_root: &Path,
    has_watcher_config: bool,
    cmd: &mut Command,
) {
    if svc.name != "watcher" || has_watcher_config {
        return;
    }

    if let Ok(explicit) = std::env::var("WATCHER_CONFIG")
        && !explicit.trim().is_empty()
    {
        cmd.env("WATCHER_CONFIG", explicit);
        return;
    }

    let mode = std::env::var("LAUNCHER_WATCHER_MODE").unwrap_or_else(|_| "prod".to_string());
    let config_rel = if matches!(mode.as_str(), "dev" | "development") {
        "projects/products/stable/core/watcher/watcher.dev.toml"
    } else {
        "projects/products/stable/core/watcher/watcher.toml"
    };
    let config_path = workspace_root.join(config_rel);
    cmd.env("WATCHER_CONFIG", config_path);
}
