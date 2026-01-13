use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use diffy::{apply, Patch};
use serde_json::json;
use walkdir::WalkDir;

use crate::{actions::ActionResult, policy::Policy};

#[derive(Clone)]
pub struct SandboxFs {
    policy: Policy,
}

impl SandboxFs {
    pub fn new(policy: Policy) -> Self {
        Self { policy }
    }

    pub fn read_file(&self, rel: &str) -> Result<ActionResult> {
        let path = self.policy.resolve_work_path_for_read(rel)?;
        let bytes = fs::read(&path).with_context(|| format!("read failed: {}", path.display()))?;

        if bytes.len() > self.policy.config().max_read_bytes {
            return Ok(ActionResult::error(
                "PolicyViolation",
                format!(
                    "read too large: {} bytes (max {})",
                    bytes.len(),
                    self.policy.config().max_read_bytes
                ),
            ));
        }

        let text = String::from_utf8_lossy(&bytes).to_string();

        Ok(ActionResult::success(
            "ReadFile",
            format!("read {} bytes", bytes.len()),
            Some(json!({ "path": rel, "contents": text, "bytes": bytes.len() })),
        ))
    }

    pub fn list_dir(&self, rel: &str, max_depth: usize) -> Result<ActionResult> {
        let root = self.policy.resolve_work_path_for_read(rel)?;
        let depth = if max_depth == 0 { 3 } else { max_depth.min(12) };

        let mut entries = Vec::new();
        for e in WalkDir::new(&root)
            .max_depth(depth)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let p = e.path();
            let is_dir = p.is_dir();

            let rel_display = p
                .strip_prefix(self.policy.work_root())
                .map(|r| r.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| p.to_string_lossy().replace('\\', "/"));

            if self
                .policy
                .config()
                .forbid_globs
                .iter()
                .any(|g| crate::policy::glob_match(&rel_display, g))
            {
                continue;
            }

            // Vérifie les droits READ
            if !self
                .policy
                .config()
                .allow_read_globs
                .iter()
                .any(|g| crate::policy::glob_match(&rel_display, g))
            {
                continue;
            }

            entries.push(json!({
                "path": rel_display,
                "is_dir": is_dir
            }));
        }

        Ok(ActionResult::success(
            "ListDir",
            format!("listed {} entries", entries.len()),
            Some(json!({ "root": rel, "entries": entries })),
        ))
    }

    pub fn write_file(&self, rel: &str, contents: &str, create_dirs: bool) -> Result<ActionResult> {
        if contents.len() > self.policy.config().max_write_bytes {
            return Ok(ActionResult::error(
                "PolicyViolation",
                format!(
                    "write too large: {} bytes (max {})",
                    contents.len(),
                    self.policy.config().max_write_bytes
                ),
            ));
        }

        let abs = self.policy.resolve_work_path_for_write(rel)?;

        if create_dirs {
            if let Some(parent) = abs.parent() {
                fs::create_dir_all(parent)?;
            }
        }

        atomic_write(&abs, contents.as_bytes())?;

        Ok(ActionResult::success(
            "WriteFile",
            "written",
            Some(json!({ "path": rel, "bytes": contents.len() })),
        ))
    }

    pub fn apply_unified_diff(&self, rel: &str, unified_diff: &str) -> Result<ActionResult> {
        let abs = self.policy.resolve_work_path_for_write(rel)?;
        let original = fs::read_to_string(&abs).unwrap_or_default();

        let patch = Patch::from_str(unified_diff)
            .map_err(|e| anyhow::anyhow!("invalid unified diff: {e}"))?;

        let updated =
            apply(&original, &patch).map_err(|e| anyhow::anyhow!("patch apply failed: {e}"))?;

        if updated.len() > self.policy.config().max_write_bytes {
            return Ok(ActionResult::error(
                "PolicyViolation",
                format!(
                    "patched file too large: {} bytes (max {})",
                    updated.len(),
                    self.policy.config().max_write_bytes
                ),
            ));
        }

        if let Some(parent) = abs.parent() {
            fs::create_dir_all(parent)?;
        }
        atomic_write(&abs, updated.as_bytes())?;

        Ok(ActionResult::success(
            "ApplyUnifiedDiff",
            "patched",
            Some(json!({ "path": rel, "bytes": updated.len() })),
        ))
    }
}

fn atomic_write(path: &PathBuf, bytes: &[u8]) -> Result<()> {
    let tmp = path.with_extension("tmp.agent");
    fs::write(&tmp, bytes).with_context(|| format!("write tmp failed: {}", tmp.display()))?;
    fs::rename(&tmp, path).with_context(|| format!("rename tmp failed: {}", path.display()))?;
    Ok(())
}
