// projects/products/unstable/platform_versioning/backend/src/slices/slice_manifest.rs
use serde::{Deserialize, Serialize};

/// A deterministic, serialisable representation of a user's allowed repository
/// paths for a specific issue.
///
/// The manifest is derived from the issue's [`super::SliceDefinition`] and is
/// used by the backend to filter all path-sensitive API responses. Paths are
/// stored in lexicographic order so that manifests can be compared and cached
/// reliably.
///
/// # Non-leakage guarantee
///
/// Paths that are not present in this manifest must never appear in any API
/// response for the associated user+issue pair. The backend enforces this
/// contract by routing all file-level responses through the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SliceManifest {
    /// The subject (user identifier) this manifest was generated for.
    pub subject: String,
    /// The issue identifier this manifest was generated from.
    pub issue_id: String,
    /// Allowed path prefixes, sorted lexicographically.
    ///
    /// A path in a repository response is visible iff it exactly matches one
    /// of these entries or starts with `"{entry}/"`.
    pub allowed_paths: Vec<String>,
}

impl SliceManifest {
    /// Returns `true` if the given `path` string is accessible through this
    /// manifest.
    pub fn allows(&self, path: &str) -> bool {
        self.allowed_paths.iter().any(|allowed| {
            path == allowed.as_str() || path.starts_with(&format!("{}/", allowed.as_str()))
        })
    }

    /// Returns `true` if no paths are allowed (empty slice).
    pub fn is_empty(&self) -> bool {
        self.allowed_paths.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest(paths: &[&str]) -> SliceManifest {
        SliceManifest {
            subject: "alice".to_string(),
            issue_id: "iss-1".to_string(),
            allowed_paths: paths.iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn exact_match_is_allowed() {
        let m = manifest(&["src/main.rs"]);
        assert!(m.allows("src/main.rs"));
    }

    #[test]
    fn child_of_dir_is_allowed() {
        let m = manifest(&["src"]);
        assert!(m.allows("src/lib.rs"));
        assert!(m.allows("src/sub/mod.rs"));
    }

    #[test]
    fn sibling_dir_is_not_allowed() {
        let m = manifest(&["src"]);
        assert!(!m.allows("tests/integration.rs"));
    }

    #[test]
    fn prefix_without_slash_does_not_match_sibling() {
        let m = manifest(&["src"]);
        assert!(!m.allows("srcfoo/main.rs"));
    }

    #[test]
    fn empty_manifest_allows_nothing() {
        let m = manifest(&[]);
        assert!(!m.allows("readme.md"));
        assert!(m.is_empty());
    }
}
