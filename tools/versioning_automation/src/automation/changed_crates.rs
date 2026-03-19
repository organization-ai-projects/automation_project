//! tools/versioning_automation/src/automation/changed_crates.rs
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use crate::automation::commands::ChangedCratesOptions;

use super::execute::{
    ensure_git_repo, find_crate_dir_for_file, repo_root, run_git_output_preserve,
};

pub(crate) fn run_changed_crates(opts: ChangedCratesOptions) -> Result<(), String> {
    ensure_git_repo()?;
    let changed_files = git_changed_files(opts.ref1.as_deref(), opts.ref2.as_deref())?;
    if changed_files.is_empty() {
        println!("No changed files.");
        return Ok(());
    }

    let repo_root = repo_root()?;
    let mut crate_paths = BTreeSet::new();
    for file in changed_files {
        if let Some(path) = find_crate_dir_for_file(&repo_root, &file) {
            crate_paths.insert(path);
        }
    }

    if crate_paths.is_empty() {
        println!("No crates affected.");
        return Ok(());
    }

    let output_paths_only = opts.output_format.as_deref() == Some("paths");
    if output_paths_only {
        for path in crate_paths {
            println!("{path}");
        }
        return Ok(());
    }

    println!("Changed crates:");
    for path in crate_paths {
        let crate_name = read_crate_name(&repo_root, &path).unwrap_or_else(|| path.clone());
        println!("  - {crate_name} ({path})");
    }
    Ok(())
}

fn git_changed_files(ref1: Option<&str>, ref2: Option<&str>) -> Result<Vec<String>, String> {
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
