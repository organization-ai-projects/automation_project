// projects/products/code_agent_sandbox/src/sandbox_fs.rs
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use common_json::pjson;
use diffy::{Patch, apply};
use rand::{TryRngCore, rngs::OsRng};
use walkdir::WalkDir;

use crate::{actions::ActionResult, policies::Policy};

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
        let metadata =
            fs::metadata(&path).with_context(|| format!("metadata failed: {}", path.display()))?;

        if metadata.len() as usize > self.policy.config().max_read_bytes {
            return Ok(ActionResult::error(
                "PolicyViolation",
                format!(
                    "read too large: {} bytes (max {})",
                    metadata.len(),
                    self.policy.config().max_read_bytes
                ),
            ));
        }

        let bytes = fs::read(&path).with_context(|| format!("read failed: {}", path.display()))?;
        let text = String::from_utf8_lossy(&bytes).to_string();

        Ok(ActionResult::success(
            "ReadFile",
            format!("read {} bytes", bytes.len()),
            Some(
                pjson!({ "path": (rel.to_string()), "contents": text, "bytes": (bytes.len() as i64) }),
            ),
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

            let rel_display = match p.strip_prefix(self.policy.work_root()) {
                Ok(r) => r.to_string_lossy().replace('\\', "/"),
                Err(_) => continue, // Skip si strip_prefix échoue
            };

            if self
                .policy
                .resolve_work_path_for_read(&rel_display)
                .is_err()
            {
                continue;
            }

            entries.push(pjson!({
                "path": rel_display,
                "is_dir": is_dir
            }));

            // Limite le nombre d'entrées
            if entries.len() >= self.policy.config().max_files_per_request {
                break;
            }
        }

        Ok(ActionResult::success(
            "ListDir",
            format!("listed {} entries", entries.len()),
            Some(pjson!({ "root": rel, "entries": entries })),
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

        if create_dirs && let Some(parent) = abs.parent() {
            fs::create_dir_all(parent)?;
        }

        // Vérification anti-symlink avant écriture
        self.validate_symlinks(&abs)?;

        atomic_write(&abs, contents.as_bytes())?;

        Ok(ActionResult::success(
            "WriteFile",
            "written",
            Some(pjson!({ "path": (rel.to_string()), "bytes": (contents.len() as i64) })),
        ))
    }

    pub fn apply_unified_diff(&self, rel: &str, unified_diff: &str) -> Result<ActionResult> {
        if unified_diff.len() > self.policy.config().max_write_bytes * 2 {
            return Ok(ActionResult::error(
                "PolicyViolation",
                format!(
                    "unified diff too large: {} bytes (max {})",
                    unified_diff.len(),
                    self.policy.config().max_write_bytes * 2
                ),
            ));
        }

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

        // Vérification anti-symlink avant écriture
        self.validate_symlinks(&abs)?;

        atomic_write(&abs, updated.as_bytes())?;

        Ok(ActionResult::success(
            "ApplyUnifiedDiff",
            "patched",
            Some(pjson!({ "path": (rel.to_string()), "bytes": (updated.len() as i64) })),
        ))
    }

    pub fn validate_symlinks(&self, path: &Path) -> Result<()> {
        // Vérifie les parents pour détecter les symlinks jusqu'à work_root
        let mut cur = path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| path.to_path_buf());

        while cur.starts_with(self.policy.work_root()) {
            if cur.exists() {
                let md = fs::symlink_metadata(&cur)?;
                if md.file_type().is_symlink() {
                    bail!("Symlink detected in path ancestry: {}", cur.display());
                }
            }
            match cur.parent() {
                Some(p) => cur = p.to_path_buf(),
                None => break,
            }
        }

        // Canonicalise le parent existant (ou work_root)
        let parent = path.parent().unwrap_or(self.policy.work_root());
        let canon_parent = fs::canonicalize(parent).unwrap_or_else(|_| parent.to_path_buf());
        if !canon_parent.starts_with(self.policy.work_root()) {
            bail!("Path traversal detected: {}", canon_parent.display());
        }

        Ok(())
    }
}

fn atomic_write(path: &PathBuf, bytes: &[u8]) -> Result<()> {
    let parent = path.parent().context("no parent dir")?;
    fs::create_dir_all(parent).ok(); // Assure que le répertoire existe

    // Génère un nom temporaire unique dans le même répertoire
    let mut tmp = parent.to_path_buf();
    let mut buf = [0u8; 8];
    OsRng
        .try_fill_bytes(&mut buf)
        .map_err(|e| anyhow::anyhow!("Failed to generate nonce: {e}"))?;
    let nonce = u64::from_be_bytes(buf);
    tmp.push(format!(
        ".tmp.agent.{}.{}",
        path.file_name().and_then(|s| s.to_str()).unwrap_or("file"),
        nonce
    ));

    // Crée un fichier temporaire de manière atomique
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&tmp)
        .with_context(|| format!("open tmp failed: {}", tmp.display()))?;

    f.write_all(bytes).context("write tmp failed")?;
    f.sync_all().ok(); // Optionnel pour la robustesse en cas de crash

    // Vérifie que la cible n'est pas un symlink
    if path
        .symlink_metadata()
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
    {
        bail!("target path is a symlink: {}", path.display());
    }

    fs::rename(&tmp, path)
        .with_context(|| format!("rename tmp failed: {} -> {}", tmp.display(), path.display()))?;

    Ok(())
}
