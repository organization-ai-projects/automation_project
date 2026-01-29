// projects/products/code_agent_sandbox/src/worktree.rs
use std::fs;

use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::policies::{Policy, policy::glob_match};

/// Initialize worktree from source repo using allowlists/forbids.
/// Idempotent: if Cargo.toml exists in work_root, it won't recopy.
pub fn init_worktree_from_repo(policy: &Policy) -> Result<()> {
    let src_root = policy.source_repo_root();
    let dst_root = policy.work_root();

    if dst_root.join("Cargo.toml").exists() {
        return Ok(());
    }

    fs::create_dir_all(dst_root).context("create work_root")?;

    let mut copied = 0usize;

    for entry in WalkDir::new(src_root).into_iter().filter_map(|e| e.ok()) {
        let p = entry.path();
        if p.is_dir() {
            continue;
        }
        let rel = match p.strip_prefix(src_root) {
            Ok(r) => r,
            Err(_) => continue,
        };

        let rel_str = rel.to_string_lossy().replace('\\', "/");

        // forbid
        let is_forbidden = policy
            .config()
            .forbid_globs
            .iter()
            .try_fold(false, |acc, g| glob_match(&rel_str, g).map(|m| acc || m))?;

        if is_forbidden {
            continue;
        }

        // allow: read or write
        let allowed_read = policy
            .config()
            .allow_read_globs
            .iter()
            .try_fold(false, |acc, g| glob_match(&rel_str, g).map(|m| acc || m))?;

        let allowed_write = policy
            .config()
            .allow_write_globs
            .iter()
            .try_fold(false, |acc, g| glob_match(&rel_str, g).map(|m| acc || m))?;

        if !(allowed_read || allowed_write) {
            continue;
        }

        let dst_path = dst_root.join(rel);

        if let Some(parent) = dst_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("create parent {}", parent.display()))?;
        }

        let bytes = fs::read(p).with_context(|| format!("read {}", p.display()))?;
        fs::write(&dst_path, &bytes).with_context(|| format!("write {}", dst_path.display()))?;
        copied += 1;
    }

    fs::write(
        dst_root.join(".worktree_initialized"),
        format!("copied_files={copied}\n"),
    )
    .ok();
    Ok(())
}
