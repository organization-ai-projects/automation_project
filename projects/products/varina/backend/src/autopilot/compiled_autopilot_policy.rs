//! projects/products/varina/backend/src/compiled_autopilot_policy.rs
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::AutopilotPolicy;

/// Compiled policy: normalized + optimized structures.
/// Goal: avoid normalizing/iterating too much on every change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledAutopilotPolicy {
    pub protected_branches: HashSet<String>,

    pub relevant_prefixes_norm: Vec<String>,
    pub relevant_files_norm: HashSet<String>,
    pub blocked_prefixes_norm: Vec<String>,

    pub fail_on_unrelated_changes: bool,

    pub allow_push: bool,
    pub push_remote: String,
    pub push_set_upstream_if_missing: bool,
}

impl From<&AutopilotPolicy> for CompiledAutopilotPolicy {
    fn from(p: &AutopilotPolicy) -> Self {
        let protected_branches = p
            .protected_branches
            .iter()
            .map(|s| s.trim().to_string())
            .collect();

        let mut relevant_prefixes_norm = normalize_prefixes(&p.relevant_prefixes);
        let blocked_prefixes_norm = normalize_prefixes(&p.blocked_prefixes);

        let relevant_files_norm = p
            .relevant_files
            .iter()
            .map(|f| normalize_path(f))
            .collect::<HashSet<_>>();

        relevant_prefixes_norm.push("projects/libraries/".to_string());
        relevant_prefixes_norm.push("projects/products/".to_string());

        // Check for the existence of paths
        relevant_prefixes_norm.retain(|prefix| {
            let exists = std::path::Path::new(prefix).exists();
            if !exists {
                println!("[warning] Prefix does not exist: {}", prefix);
            }
            exists
        });

        println!(
            "[debug] relevant_prefixes_norm: {:?}",
            relevant_prefixes_norm
        );

        Self {
            protected_branches,
            relevant_prefixes_norm,
            relevant_files_norm,
            blocked_prefixes_norm,
            fail_on_unrelated_changes: p.fail_on_unrelated_changes,
            allow_push: p.allow_push,
            push_remote: p.push_remote.clone(),
            push_set_upstream_if_missing: p.push_set_upstream_if_missing,
        }
    }
}

/// Normalize separators and trivial prefixes (shared).
pub fn normalize_path(p: &str) -> String {
    let mut s = p.trim().replace('\\', "/");

    while s.contains("//") {
        s = s.replace("//", "/");
    }

    while s.starts_with("./") {
        s = s[2..].to_string();
    }

    if s.len() > 1 && s.ends_with('/') {
        s.pop();
    }

    s
}

/// Ensure prefixes behave like directory prefixes:
/// - normalize
/// - ensure trailing "/" so boundary checks are consistent
fn normalize_prefixes(prefixes: &[String]) -> Vec<String> {
    let mut out = Vec::with_capacity(prefixes.len());
    for pref in prefixes {
        let mut n = normalize_path(pref);

        // If user wrote "src" we treat it as "src/" (directory prefix)
        if !n.is_empty() && !n.ends_with('/') {
            n.push('/');
        }

        if !n.is_empty() {
            let absolute_path = std::env::current_dir().unwrap().join(&n);
            if !absolute_path.exists() {
                println!(
                    "[warning] normalize_prefixes: Absolute prefix does not exist: {}",
                    absolute_path.display()
                );
                continue;
            }
            println!("[debug] normalize_prefixes: Adding normalized prefix={}", n);
            out.push(n);
        }
    }

    // Optional: sort longest-first so specific prefixes match earlier (if you ever care)
    out.sort_by_key(|b| std::cmp::Reverse(b.len()));
    out
}

/// Prefix match with boundary:
/// - We assume prefix ends with '/' (compiled), so a simple starts_with is safe.
pub fn path_has_compiled_prefix(path_norm: &str, prefix_norm: &str) -> bool {
    if prefix_norm.is_empty() {
        return false;
    }
    println!(
        "[debug] path_has_compiled_prefix: Checking if path_norm={} starts with prefix_norm={}",
        path_norm, prefix_norm
    );
    path_norm.starts_with(prefix_norm)
}
