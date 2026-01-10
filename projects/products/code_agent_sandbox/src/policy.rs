// projects/products/code_agent_sandbox/src/policy.rs
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Policy {
    cfg: PolicyConfig,
    repo_root_canon: PathBuf,
    run_dir_canon: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    pub repo_root: PathBuf,
    pub run_dir: PathBuf,

    pub max_read_bytes: usize,
    pub max_write_bytes: usize,
    pub max_files_per_request: usize,

    /// Deny patterns (simple glob-ish with "**" and "*" only).
    pub forbid_globs: Vec<String>,
    /// Allow writes only to these.
    pub allow_write_globs: Vec<String>,
    /// Allow reads to these (still filtered by forbid_globs).
    pub allow_read_globs: Vec<String>,
}

impl Policy {
    pub fn new(cfg: PolicyConfig) -> Result<Self> {
        let repo_root_canon =
            canonical_dir(&cfg.repo_root).context("repo_root must exist and be a directory")?;
        let run_dir_canon = canonical_dir(&cfg.run_dir)
            .or_else(|_| {
                std::fs::create_dir_all(&cfg.run_dir)?;
                canonical_dir(&cfg.run_dir)
            })
            .context("run_dir invalid")?;

        Ok(Self {
            cfg,
            repo_root_canon,
            run_dir_canon,
        })
    }

    pub fn config(&self) -> &PolicyConfig {
        &self.cfg
    }

    pub fn repo_root(&self) -> &Path {
        &self.repo_root_canon
    }

    pub fn run_dir(&self) -> &Path {
        &self.run_dir_canon
    }

    /// Resolve a repo-relative path (like "src/main.rs") into a canonical absolute path
    /// and enforce allow/deny rules.
    pub fn resolve_repo_path_for_read(&self, rel: &str) -> Result<PathBuf> {
        let abs = self.safe_join(self.repo_root(), rel)?;
        self.enforce_globs(
            rel,
            &self.cfg.allow_read_globs,
            &self.cfg.forbid_globs,
            "read",
        )?;
        Ok(abs)
    }

    pub fn resolve_repo_path_for_write(&self, rel: &str) -> Result<PathBuf> {
        let abs = self.safe_join(self.repo_root(), rel)?;
        self.enforce_globs(
            rel,
            &self.cfg.allow_write_globs,
            &self.cfg.forbid_globs,
            "write",
        )?;
        Ok(abs)
    }

    fn safe_join(&self, root: &Path, rel: &str) -> Result<PathBuf> {
        if rel.contains('\0') {
            bail!("invalid path (NUL byte)");
        }

        // Very defensive: forbid absolute paths and parent traversal.
        let rel_path = Path::new(rel);
        if rel_path.is_absolute() {
            bail!("absolute paths forbidden: {rel}");
        }
        if rel_path
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            bail!("parent traversal forbidden: {rel}");
        }

        let joined = root.join(rel_path);

        // Canonicalize parent dir if file doesn't exist yet.
        let canon = if joined.exists() {
            joined
                .canonicalize()
                .with_context(|| format!("cannot canonicalize: {rel}"))?
        } else {
            let parent = joined.parent().context("path has no parent")?;
            let parent_canon = parent
                .canonicalize()
                .with_context(|| format!("cannot canonicalize parent for: {rel}"))?;
            parent_canon.join(joined.file_name().context("invalid file name")?)
        };

        // Ensure it stays inside root.
        let root = root.canonicalize()?;
        if !canon.starts_with(&root) {
            bail!("path escapes root: {rel}");
        }
        Ok(canon)
    }

    fn enforce_globs(&self, rel: &str, allow: &[String], deny: &[String], op: &str) -> Result<()> {
        let rel_norm = rel.replace('\\', "/");

        if deny.iter().any(|g| glob_match(&rel_norm, g)) {
            bail!("policy forbids {op} on: {rel}");
        }

        if !allow.is_empty() && !allow.iter().any(|g| glob_match(&rel_norm, g)) {
            bail!("policy does not allow {op} on: {rel}");
        }

        Ok(())
    }
}

fn canonical_dir(p: &Path) -> Result<PathBuf> {
    let meta = std::fs::metadata(p)?;
    if !meta.is_dir() {
        bail!("not a directory: {}", p.display());
    }
    Ok(p.canonicalize()?)
}

/// Minimal glob matcher:
/// - "*" matches any chars except "/"
/// - "**" matches any chars including "/"
///
/// This is enough for policy gates without pulling a heavy glob engine.
pub fn glob_match(path: &str, glob: &str) -> bool {
    let p = path;
    let g = glob.replace('\\', "/");

    fn rec(p: &str, g: &str) -> bool {
        if g.is_empty() {
            return p.is_empty();
        }
        if g == "**" {
            return true;
        }
        if let Some(rest) = g.strip_prefix("**/") {
            // "**/" can match empty or any nested dirs
            if rec(p, rest) {
                return true;
            }
            if let Some(pos) = p.find('/') {
                return rec(&p[pos + 1..], g);
            }
            return false;
        }
        if let Some(rest) = g.strip_prefix('*') {
            // '*' matches any segment chars (not '/')
            // try all possible consumptions until '/' boundary
            let mut i = 0usize;
            while i <= p.len() {
                if i < p.len() && &p[i..i + 1] == "/" {
                    break;
                }
                if rec(&p[i..], rest) {
                    return true;
                }
                i += 1;
            }
            return false;
        }
        // literal char
        if let (Some(pc), Some(gc)) = (p.chars().next(), g.chars().next()) {
            if pc == gc {
                return rec(&p[pc.len_utf8()..], &g[gc.len_utf8()..]);
            }
        }
        false
    }

    rec(p, &g)
}
