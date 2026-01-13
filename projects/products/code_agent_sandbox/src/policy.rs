// projects/products/code_agent_sandbox/src/policy.rs
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use toml::Value;

use crate::policy_config::PolicyConfig;

#[derive(Clone)]
pub struct Policy {
    cfg: PolicyConfig,
}

impl Policy {
    pub fn new(cfg: PolicyConfig) -> Result<Self> {
        Ok(Self { cfg })
    }

    pub fn config(&self) -> &PolicyConfig {
        &self.cfg
    }

    pub fn source_repo_root(&self) -> &Path {
        &self.cfg.source_repo_root
    }

    pub fn work_root(&self) -> &Path {
        &self.cfg.work_root
    }

    pub fn run_dir(&self) -> &Path {
        &self.cfg.run_dir
    }

    pub fn resolve_work_path_for_read(&self, rel: &str) -> Result<PathBuf> {
        self.resolve_work_path(rel, AccessKind::Read)
    }

    pub fn resolve_work_path_for_write(&self, rel: &str) -> Result<PathBuf> {
        self.resolve_work_path(rel, AccessKind::Write)
    }

    fn is_allowed(&self, rel_norm: &str, kind: AccessKind) -> bool {
        match kind {
            AccessKind::Read => self
                .cfg
                .allow_read_globs
                .iter()
                .any(|g| glob_match(rel_norm, g)),
            AccessKind::Write => self
                .cfg
                .allow_write_globs
                .iter()
                .any(|g| glob_match(rel_norm, g)),
        }
    }

    fn resolve_work_path(&self, rel: &str, kind: AccessKind) -> Result<PathBuf> {
        let rel_norm = normalize_rel(rel);

        // Reject paths with invalid segments
        if rel_norm
            .split('/')
            .any(|seg| seg == ".." || seg.is_empty() || seg == ".")
        {
            bail!("invalid relative path: {rel_norm}");
        }

        // Vérifie d'abord les chemins interdits
        if self
            .cfg
            .forbid_globs
            .iter()
            .any(|g| glob_match(&rel_norm, g))
        {
            bail!("forbidden path by policy: {rel_norm}");
        }

        // Applique la logique deny-by-default
        if !self.is_allowed(&rel_norm, kind) {
            bail!("access not allowed by policy: {rel_norm}");
        }

        let abs = self.cfg.work_root.join(&rel_norm);

        // Protection contre la traversée de répertoires
        let canon_root =
            std::fs::canonicalize(&self.cfg.work_root).unwrap_or(self.cfg.work_root.clone());
        let parent = abs.parent().unwrap_or(&abs);
        let canon_parent = std::fs::canonicalize(parent).unwrap_or_else(|_| parent.to_path_buf());
        if !canon_parent.starts_with(&canon_root) {
            bail!("path traversal detected: {rel_norm}");
        }

        Ok(abs)
    }

    pub fn load_with_overrides(cfg: PolicyConfig, overrides_path: &Path) -> Result<Self> {
        let mut policy = Self::new(cfg)?;

        if overrides_path.exists() {
            let content = fs::read_to_string(overrides_path)
                .context("Failed to read policy_overrides.toml")?;
            let overrides: Value =
                toml::from_str(&content).context("Failed to parse policy_overrides.toml")?;

            if let Some(forbid) = overrides.get("forbid_globs").and_then(|v| v.as_array()) {
                for glob in forbid.iter().filter_map(|v| v.as_str()) {
                    policy.cfg.forbid_globs.push(glob.to_string());
                    log::info!("OVERRIDE: Added to forbid_globs: {}", glob);
                }
            }

            if overrides.get("allow_read_globs").is_some()
                || overrides.get("allow_write_globs").is_some()
            {
                log::warn!("OVERRIDE: Allow rules in overrides are ignored for safety.");
            }
        }

        Ok(policy)
    }
}

#[derive(Copy, Clone)]
enum AccessKind {
    Read,
    Write,
}

fn normalize_rel(rel: &str) -> String {
    let mut s = rel.trim().replace('\\', "/");
    while s.starts_with("./") {
        s = s[2..].to_string();
    }
    while s.starts_with('/') {
        s = s[1..].to_string();
    }
    s
}

/// IMPORTANT: you already have this function used by SandboxFs.
/// Keep your existing implementation if you have one.
/// This fallback is minimal.
pub fn glob_match(path: &str, pattern: &str) -> bool {
    if pattern == "**" || pattern == "**/**" {
        return true;
    }
    if let Some(prefix) = pattern.strip_suffix("/**") {
        return path == prefix || path.starts_with(&format!("{prefix}/"));
    }
    if pattern.contains('*') {
        // Refuse ambiguous patterns in a sandbox
        return false;
    }
    path == pattern
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forbid_wins_over_allow() {
        let cfg = PolicyConfig {
            source_repo_root: PathBuf::from("/repo"),
            work_root: PathBuf::from("/work"),
            run_dir: PathBuf::from("/run"),
            max_read_bytes: 1024,
            max_write_bytes: 1024,
            max_files_per_request: 10,
            forbid_globs: vec!["src/forbidden/**".into()],
            allow_read_globs: vec!["src/**".into()],
            allow_write_globs: vec![],
        };

        let policy = Policy::new(cfg).unwrap();

        // Path is allowed by `allow_read_globs` but forbidden by `forbid_globs`
        let result = policy.resolve_work_path("src/forbidden/file.txt", AccessKind::Read);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("forbidden path by policy"));
    }
}
