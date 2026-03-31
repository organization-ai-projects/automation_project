//! tools/versioning_automation/src/automation/changed_crates.rs
use std::fs;
use std::path::Path;

use super::execute::run_git_output_preserve;

pub(crate) fn git_changed_files(
    ref1: Option<&str>,
    ref2: Option<&str>,
) -> Result<Vec<String>, String> {
    let output = match (ref1, ref2) {
        (Some(a), Some(b)) => run_git_output_preserve(&["diff", "--name-only", a, b])?,
        (Some(a), None) => run_git_output_preserve(&["diff", "--name-only", a, "HEAD"])?,
        (None, None) => run_git_output_preserve(&["diff", "--name-only", "HEAD"])?,
        (None, Some(_)) => {
            return Err("Second ref provided without first ref.".to_string());
        }
    };
    Ok(output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .collect())
}

pub(crate) fn read_crate_name(repo_root: &Path, crate_path: &str) -> Option<String> {
    let cargo_toml = repo_root.join(crate_path).join("Cargo.toml");
    let content = fs::read_to_string(cargo_toml).ok()?;
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("name") {
            let rhs = rest.trim_start();
            if let Some(value) = rhs.strip_prefix('=') {
                let raw = value.trim();
                if raw.starts_with('"') && raw.ends_with('"') && raw.len() >= 2 {
                    return Some(raw[1..raw.len() - 1].to_string());
                }
            }
        }
    }
    None
}
