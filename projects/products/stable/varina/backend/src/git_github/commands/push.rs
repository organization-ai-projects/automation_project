//! projects/products/varina/backend/src/git_github/commands/push.rs
use std::{path::Path, process};

use crate::autopilot::AutopilotError;

/// Pushes the current branch to the remote repository.
pub fn git_push_current_branch(
    repo_path: &Path,
    branch: &str,
    force: bool,
    logs: &mut Vec<String>,
) -> Result<(), AutopilotError> {
    let mut command = process::Command::new("git");
    command
        .arg("-C")
        .arg(repo_path)
        .arg("push")
        .arg("origin")
        .arg(branch);

    if force {
        command.arg("--force");
    }

    let output = command
        .output()
        .map_err(|e| AutopilotError::from(format!("Failed to execute git push: {}", e)))?;

    if !output.status.success() {
        return Err(AutopilotError::from(format!(
            "Git push failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    logs.push(format!("[git] Pushed branch: {}", branch));
    Ok(())
}
