//! tools/versioning_automation/src/automation/install_hooks/mod.rs
use std::path::{Path, PathBuf};
use std::{env, fs};

use crate::automation::{commands::InstallHooksOptions, execute};

pub(crate) fn run_install_hooks(_opts: InstallHooksOptions) -> Result<(), String> {
    execute::ensure_git_repo()?;
    let root = execute::repo_root()?;
    let hooks_dir = execute::git_hooks_dir(&root)?;
    fs::create_dir_all(&hooks_dir).map_err(|e| {
        format!(
            "Failed to create hooks directory '{}': {e}",
            hooks_dir.display()
        )
    })?;

    println!("Installing hooks to '{}'", hooks_dir.display());
    let binary = resolve_versioning_automation_binary(&root)?;
    let hooks = [
        "pre-commit",
        "prepare-commit-msg",
        "commit-msg",
        "pre-push",
        "post-checkout",
        "pre-branch-create",
        "branch-creation-check",
    ];
    for hook in hooks {
        install_hook_entrypoint(&hooks_dir, hook, &binary)?;
    }

    println!("Hooks installed successfully.");
    Ok(())
}

fn resolve_versioning_automation_binary(root: &Path) -> Result<PathBuf, String> {
    if let Ok(path) = env::var("VERSIONING_AUTOMATION_BIN") {
        let candidate = PathBuf::from(path);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }

    let current = env::current_exe()
        .map_err(|e| format!("Failed to resolve current executable path: {e}"))?;
    if current.is_file() {
        return Ok(current);
    }

    let candidates = [
        root.join("target/debug/versioning_automation"),
        root.join("target/release/versioning_automation"),
    ];
    for candidate in candidates {
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    Err("Unable to resolve versioning_automation binary path.".to_string())
}

fn install_hook_entrypoint(hooks_dir: &Path, hook_name: &str, binary: &Path) -> Result<(), String> {
    let target = hooks_dir.join(hook_name);
    if target.exists() {
        fs::remove_file(&target)
            .map_err(|e| format!("Failed to remove existing hook '{}': {e}", target.display()))?;
    }

    if link_hook_symlink(binary, &target).is_ok() {
        println!("✅ Installed {hook_name} (symlink)");
        return Ok(());
    }
    if fs::hard_link(binary, &target).is_ok() {
        println!("✅ Installed {hook_name} (hardlink)");
        return Ok(());
    }
    fs::copy(binary, &target).map_err(|e| {
        format!(
            "Failed to copy binary '{}' to hook '{}': {e}",
            binary.display(),
            target.display()
        )
    })?;
    ensure_hook_executable_permissions(&target)?;
    println!("✅ Installed {hook_name} (copy)");
    Ok(())
}

#[cfg(unix)]
fn ensure_hook_executable_permissions(path: &Path) -> Result<(), String> {
    let mut perms = fs::metadata(path)
        .map_err(|e| format!("Failed to read '{}': {e}", path.display()))?
        .permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
    fs::set_permissions(path, perms)
        .map_err(|e| format!("Failed to chmod '{}': {e}", path.display()))
}

#[cfg(not(unix))]
fn ensure_hook_executable_permissions(_path: &Path) -> Result<(), String> {
    Ok(())
}

#[cfg(unix)]
fn link_hook_symlink(binary: &Path, target: &Path) -> Result<(), String> {
    std::os::unix::fs::symlink(binary, target).map_err(|e| e.to_string())
}

#[cfg(windows)]
fn link_hook_symlink(binary: &Path, target: &Path) -> Result<(), String> {
    std::os::windows::fs::symlink_file(binary, target).map_err(|e| e.to_string())
}

#[cfg(not(any(unix, windows)))]
fn link_hook_symlink(binary: &Path, target: &Path) -> Result<(), String> {
    let _ = (binary, target);
    Err("symlink not supported on this platform".to_string())
}
