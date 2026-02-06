//! projects/products/varina/backend/src/git_github/commands/reset.rs
use crate::autopilot::AutopilotError;
use std::{path::Path, process, result};
/// Resets specified files in the Git index
pub fn git_reset(
    repo_path: &Path,
    paths: &[String],
    logs: &mut Vec<String>,
) -> result::Result<(), AutopilotError> {
    logs.push(format!("[cmd] git reset -- {:?}", paths));

    let status = process::Command::new("git")
        .arg("reset")
        .arg("--")
        .args(paths)
        .current_dir(repo_path)
        .status()
        .map_err(|e| AutopilotError::from(format!("Error executing git reset: {}", e)))?;

    if !status.success() {
        return Err(AutopilotError::from("git reset failed".to_string()));
    }

    logs.push("[cmd] git reset completed successfully".into());
    Ok(())
}
