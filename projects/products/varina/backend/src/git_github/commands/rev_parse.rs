//! projects/products/varina/backend/src/git_github/commands/rev_parse.rs
use crate::AutopilotError;
use std::path::Path;
/// Ensures that the given path is a valid Git repository.
pub fn ensure_git_repo(repo_path: &Path, logs: &mut Vec<String>) -> Result<(), AutopilotError> {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .map_err(|e| AutopilotError::from(format!("Failed to execute git command: {}", e)))?;

    if !output.status.success() {
        return Err(AutopilotError::from(format!(
            "Not a valid Git repository: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    logs.push(format!(
        "[git] Verified Git repository at: {}",
        repo_path.display()
    ));
    Ok(())
}
