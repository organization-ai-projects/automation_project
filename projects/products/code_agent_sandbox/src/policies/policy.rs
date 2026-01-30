// projects/products/code_agent_sandbox/src/policies/policy.rs
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use toml::Value;

use crate::access_kind::AccessKind;
use crate::actions::{self, Action};
use crate::normalization::normalize_rel;
use crate::policies::PolicyConfig;

#[derive(Clone)]
pub(crate) struct Policy {
    pub(crate) cfg: PolicyConfig,
}

impl Policy {
    pub(crate) fn new(cfg: PolicyConfig) -> Result<Self> {
        Ok(Self { cfg })
    }

    pub(crate) fn config(&self) -> &PolicyConfig {
        &self.cfg
    }

    pub(crate) fn source_repo_root(&self) -> &Path {
        &self.cfg.context.source_repo_root
    }

    pub(crate) fn work_root(&self) -> &Path {
        &self.cfg.context.paths.work_root
    }

    pub(crate) fn resolve_work_path_for_read(&self, rel: &str) -> Result<PathBuf> {
        self.resolve_work_path(rel, AccessKind::Read)
    }

    pub(crate) fn resolve_work_path_for_write(&self, rel: &str) -> Result<PathBuf> {
        self.resolve_work_path(rel, AccessKind::Write)
    }

    fn is_allowed(&self, rel_norm: &str, kind: AccessKind) -> Result<bool> {
        match kind {
            AccessKind::Read => {
                for g in &self.cfg.allow_read_globs {
                    if glob_match(rel_norm, g)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            AccessKind::Write => {
                for g in &self.cfg.allow_write_globs {
                    if glob_match(rel_norm, g)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
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

        // First checks forbidden paths
        for g in &self.cfg.forbid_globs {
            if glob_match(&rel_norm, g)? {
                bail!("forbidden path by policy: {rel_norm}");
            }
        }

        // Applies the deny-by-default logic
        if !self.is_allowed(&rel_norm, kind)? {
            bail!("access not allowed by policy: {rel_norm}");
        }

        let abs = self.cfg.context.paths.work_root.join(&rel_norm);

        // Protection against directory traversal
        let canon_root = std::fs::canonicalize(&self.cfg.context.paths.work_root)
            .unwrap_or(self.cfg.context.paths.work_root.clone());
        let parent = abs.parent().unwrap_or(&abs);
        let canon_parent = std::fs::canonicalize(parent).unwrap_or_else(|_| parent.to_path_buf());
        if !canon_parent.starts_with(&canon_root) {
            bail!("path traversal detected: {rel_norm}");
        }

        Ok(abs)
    }

    pub(crate) fn load_with_overrides(cfg: PolicyConfig, overrides_path: &Path) -> Result<Self> {
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

    /// Authorizes an action based on the current policy rules.
    /// Returns an error if the action violates the policy.
    pub(crate) fn authorize_action(&self, action: &actions::Action) -> anyhow::Result<()> {
        let err = |e: anyhow::Error| e.context("Policy authorization failed");

        match action {
            Action::ReadFile { path } => {
                self.resolve_work_path_for_read(path)
                    .map(|_| ())
                    .map_err(err)?;
            }
            Action::ListDir { path, .. } => {
                self.resolve_work_path_for_read(path)
                    .map(|_| ())
                    .map_err(err)?;
            }
            Action::WriteFile { path, .. } => {
                self.resolve_work_path_for_write(path)
                    .map(|_| ())
                    .map_err(err)?;
            }
            Action::ApplyUnifiedDiff { path, .. } => {
                self.resolve_work_path_for_write(path)
                    .map(|_| ())
                    .map_err(err)?;
            }
            Action::RunCargo { .. } => {
                // Optional: validate cargo subcommands if needed.
            }
            Action::GenerateCode { .. } => {
                // No specific FS access required; allow by default.
            }
        }

        Ok(())
    }
}

/// IMPORTANT: you already have this function used by SandboxFs.
/// Keep your existing implementation if you have one.
/// This fallback is minimal.
pub fn glob_match(path: &str, pattern: &str) -> Result<bool> {
    use globset::{Glob, GlobMatcher};

    let glob = Glob::new(pattern).context("Invalid glob pattern")?;
    let matcher: GlobMatcher = glob.compile_matcher();

    Ok(matcher.is_match(path))
}
