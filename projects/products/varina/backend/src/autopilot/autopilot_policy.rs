// projects/products/varina/backend/src/autopilot/autopilot_policy.rs
use serde::{Deserialize, Serialize};

use crate::pre_checks::PreChecks;

/// Security policy for the autopilot.
/// Idea: the "AI" code does not decide arbitrarily; it applies a deterministic policy.
/// Defines the security rules and policies applied by the autopilot.
/// These rules determine relevant files, protected branches, and authorized actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutopilotPolicy {
    /// Branches where direct commits are forbidden.
    pub protected_branches: Vec<String>,

    /// Pre-checks to execute before acting.
    pub pre_checks: PreChecks,

    /// What is considered "relevant" (allowlist).
    /// Example: ["src/", "tests/", "crates/"]
    pub relevant_prefixes: Vec<String>,

    /// Exact files considered relevant even outside prefixes.
    /// Example: ["Cargo.toml", "Cargo.lock", "README.md"]
    pub relevant_files: Vec<String>,

    /// Anything matching here is denied (even if relevant).
    /// Example: ["target/", ".env", ".automation_project/secrets"]
    pub blocked_prefixes: Vec<String>,

    /// If true: if unrelated files exist, refuse to act.
    /// (recommended to avoid automatic split/branch behavior.)
    pub fail_on_unrelated_changes: bool,

    /// Allow automatic push.
    pub allow_push: bool,

    /// Remote to use if push is allowed. (e.g., "origin")
    pub push_remote: String,

    /// If push is allowed, push the current branch upstream if missing.
    pub push_set_upstream_if_missing: bool,
}

impl Default for AutopilotPolicy {
    fn default() -> Self {
        Self {
            protected_branches: vec!["main".into(), "dev".into()],
            pre_checks: PreChecks::FmtCheckAndTests,
            relevant_prefixes: vec!["src/".into(), "tests/".into()],
            relevant_files: vec!["Cargo.toml".into(), "Cargo.lock".into()],
            blocked_prefixes: vec!["target/".into(), ".env".into()],
            fail_on_unrelated_changes: true,
            allow_push: false,
            push_remote: "origin".into(),
            push_set_upstream_if_missing: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_policy() {
        let policy = AutopilotPolicy::default();
        assert!(policy.protected_branches.contains(&"main".to_string()));
        assert!(policy.protected_branches.contains(&"dev".to_string()));
        assert!(policy.fail_on_unrelated_changes);
        assert!(!policy.allow_push);
    }
}
