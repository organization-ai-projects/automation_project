use std::fs;
use std::path::Path;

use crate::automation::commands::CleanArtifactsOptions;

use super::execute::{ensure_git_repo, repo_root, run_command_status};

pub(crate) fn run_clean_artifacts(opts: CleanArtifactsOptions) -> Result<(), String> {
    ensure_git_repo()?;
    let root = repo_root()?;

    remove_dir_if_exists(&root.join("target"))?;
    remove_named_dirs_under(&root.join("projects"), "ui_dist")?;
    if opts.include_node_modules {
        remove_named_dirs_under(&root, "node_modules")?;
    }
    remove_nested_cargo_locks(&root.join("projects"), &root.join("Cargo.lock"))?;
    remove_files_by_suffixes(&root, &[".profraw", ".gcda", ".gcno", "~", ".bak", ".tmp"])?;

    run_command_status("cargo", &["clean"], false)?;
    println!("Build artifacts cleaned successfully.");
    Ok(())
}

fn remove_dir_if_exists(path: &Path) -> Result<(), String> {
    if path.exists() {
        fs::remove_dir_all(path)
            .map_err(|e| format!("Failed to remove directory '{}': {e}", path.display()))?;
    }
    Ok(())
}

fn remove_named_dirs_under(root: &Path, target_name: &str) -> Result<(), String> {
    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(_) => return Ok(()),
    };
    for entry in entries {
        let entry =
            entry.map_err(|e| format!("Failed to read entry under '{}': {e}", root.display()))?;
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_dir() {
            if path.file_name().and_then(|v| v.to_str()) == Some(target_name) {
                let _ = fs::remove_dir_all(&path);
            } else {
                remove_named_dirs_under(&path, target_name)?;
            }
        }
    }
    Ok(())
}

fn remove_nested_cargo_locks(projects_root: &Path, root_lock: &Path) -> Result<(), String> {
    if !projects_root.exists() {
        return Ok(());
    }
    remove_nested_cargo_locks_recursive(projects_root, root_lock)
}

fn remove_nested_cargo_locks_recursive(dir: &Path, root_lock: &Path) -> Result<(), String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory '{}': {e}", dir.display()))?;
    for entry in entries {
        let entry =
            entry.map_err(|e| format!("Failed to read entry under '{}': {e}", dir.display()))?;
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_dir() {
            remove_nested_cargo_locks_recursive(&path, root_lock)?;
            continue;
        }
        if !file_type.is_file() {
            continue;
        }
        if path.file_name().and_then(|v| v.to_str()) == Some("Cargo.lock") && path != root_lock {
            let _ = fs::remove_file(path);
        }
    }
    Ok(())
}

fn remove_files_by_suffixes(root: &Path, suffixes: &[&str]) -> Result<(), String> {
    if suffixes.is_empty() || !root.exists() {
        return Ok(());
    }
    remove_files_by_suffixes_recursive(root, suffixes)
}

fn remove_files_by_suffixes_recursive(dir: &Path, suffixes: &[&str]) -> Result<(), String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory '{}': {e}", dir.display()))?;
    for entry in entries {
        let entry =
            entry.map_err(|e| format!("Failed to read entry under '{}': {e}", dir.display()))?;
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_dir() {
            remove_files_by_suffixes_recursive(&path, suffixes)?;
            continue;
        }
        if !file_type.is_file() {
            continue;
        }
        let Some(path_text) = path.to_str() else {
            continue;
        };
        if suffixes.iter().any(|suffix| path_text.ends_with(suffix)) {
            let _ = fs::remove_file(&path);
        }
    }
    Ok(())
}
