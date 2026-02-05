//! projects/products/varina/backend/src/git_github/commands/status.rs
use std::{path, process};

use crate::autopilot::AutopilotError;

/// Retrieves the current status of the repository in a porcelain format.
pub fn git_status_porcelain_z(
    repo_path: &path::Path,
    logs: &mut Vec<String>,
) -> Result<Vec<String>, AutopilotError> {
    let output = process::Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("status")
        .arg("--porcelain=v1")
        .output()
        .map_err(|e| AutopilotError::from(format!("Failed to execute git status: {}", e)))?;

    if !output.status.success() {
        return Err(AutopilotError::from(format!(
            "Git status failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    let status_output = String::from_utf8_lossy(&output.stdout);
    logs.push(format!("[git] Status output: {}", status_output));

    let changes = status_output.lines().map(|line| line.to_string()).collect();

    Ok(changes)
}
