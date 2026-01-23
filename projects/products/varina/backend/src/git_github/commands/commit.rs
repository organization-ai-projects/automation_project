//! projects/products/varina/backend/src/git_github/commands/commit.rs
use std::{path::Path, process};

use crate::AutopilotError;

/// Commits the staged changes in the Git repository with the given subject and body.
pub fn git_commit(
    repo_path: &Path,
    subject: &str,
    body: &str,
    logs: &mut Vec<String>,
) -> Result<(), AutopilotError> {
    let mut command = process::Command::new("git");
    command
        .arg("-C")
        .arg(repo_path)
        .arg("commit")
        .arg("-m")
        .arg(subject);

    if !body.is_empty() {
        command.arg("-m").arg(body);
    }

    let output = command
        .output()
        .map_err(|e| AutopilotError::from(format!("Failed to execute git commit: {}", e)))?;

    if !output.status.success() {
        return Err(AutopilotError::from(format!(
            "Git commit failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    logs.push(format!("[git] Commit created with subject: {}", subject));
    Ok(())
}

/// Simulates a Git commit without actually creating one.
pub fn git_commit_dry_run(
    repo_path: &Path,
    subject: &str,
    body: &str,
    logs: &mut Vec<String>,
) -> Result<(), AutopilotError> {
    let mut command = process::Command::new("git");
    command
        .arg("-C")
        .arg(repo_path)
        .arg("commit")
        .arg("--dry-run")
        .arg("-m")
        .arg(subject);

    if !body.is_empty() {
        command.arg("-m").arg(body);
    }

    let output = command.output().map_err(|e| {
        AutopilotError::from(format!("Failed to execute git commit --dry-run: {}", e))
    })?;

    if !output.status.success() {
        return Err(AutopilotError::from(format!(
            "Git commit --dry-run failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    logs.push(format!(
        "[git] Dry-run commit simulated with subject: {}",
        subject
    ));
    Ok(())
}
