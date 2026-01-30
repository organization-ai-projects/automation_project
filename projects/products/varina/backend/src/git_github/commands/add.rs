//! projects/products/varina/backend/src/git_github/commands/add.rs
use std::{path::Path, process};

use crate::autopilot::AutopilotError;
// Adds the specified paths to the Git index.
pub fn git_add_paths(
    repo_path: &Path,
    paths: &[String],
    logs: &mut Vec<String>,
) -> Result<(), AutopilotError> {
    let mut command = process::Command::new("git");
    command.arg("-C").arg(repo_path).arg("add");

    for path in paths {
        command.arg(path);
    }

    let output = command
        .output()
        .map_err(|e| AutopilotError::from(format!("Failed to execute git add: {}", e)))?;

    if !output.status.success() {
        return Err(AutopilotError::from(format!(
            "Git add failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    logs.push(format!("[git] Added paths: {:?}", paths));
    Ok(())
}
