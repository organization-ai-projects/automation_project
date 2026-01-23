//! projects/products/varina/backend/src/git_github/commands/branch.rs
use std::path::Path;

use crate::AutopilotError;

// Retrieves the current branch name and checks if the HEAD is detached.
pub fn current_branch(
    repo_path: &Path,
    logs: &mut Vec<String>,
) -> Result<(String, bool), AutopilotError> {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .map_err(|e| AutopilotError::from(format!("Failed to execute git command: {}", e)))?;

    if !output.status.success() {
        return Err(AutopilotError::from(format!(
            "Failed to retrieve current branch: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let detached_head = branch == "HEAD";

    logs.push(format!(
        "[git] Current branch: {} (detached: {})",
        branch, detached_head
    ));
    Ok((branch, detached_head))
}
