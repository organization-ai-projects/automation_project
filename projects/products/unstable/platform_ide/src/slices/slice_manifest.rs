// projects/products/unstable/platform_ide/src/slices/slice_manifest.rs
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::errors::IdeError;

use super::allowed_path::{AllowedPath, is_safe_path};

/// The set of file paths the current user is allowed to access for a given
/// issue.
///
/// The manifest is authoritative: any path not listed here must be treated as
/// forbidden, even if the underlying platform API might otherwise expose it.
/// This enforces least-privilege visibility at the IDE level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliceManifest {
    /// The issue/repository identifier this manifest belongs to.
    pub issue_id: String,
    /// The commit identifier from which this slice was derived.
    pub base_commit: String,
    /// The set of file paths the user is allowed to view and edit.
    paths: HashSet<String>,
}

impl SliceManifest {
    /// Creates a new `SliceManifest` for the given issue.
    pub fn new(
        issue_id: impl Into<String>,
        base_commit: impl Into<String>,
        paths: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            issue_id: issue_id.into(),
            base_commit: base_commit.into(),
            paths: paths.into_iter().map(|p| p.into()).collect(),
        }
    }

    /// Validates and returns an [`AllowedPath`] if `raw` is in this manifest.
    ///
    /// Returns [`IdeError::PathNotAllowed`] if:
    /// - the path contains unsafe components (traversal, absolute), or
    /// - the path is not present in the manifest.
    ///
    /// The error message deliberately does not reveal which paths _are_ allowed
    /// or any details about the forbidden path.
    pub fn allow(&self, raw: &str) -> Result<AllowedPath, IdeError> {
        if !is_safe_path(raw) {
            return Err(IdeError::PathNotAllowed);
        }
        if !self.paths.contains(raw) {
            return Err(IdeError::PathNotAllowed);
        }
        Ok(AllowedPath::new_validated(raw.to_string()))
    }

    /// Returns an iterator over the allowed paths in this manifest.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.paths.iter().map(|s| s.as_str())
    }

    /// Returns the number of allowed paths.
    pub fn len(&self) -> usize {
        self.paths.len()
    }

    /// Returns `true` if the manifest contains no paths.
    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest() -> SliceManifest {
        SliceManifest::new("issue-42", "abc123", ["src/main.rs", "README.md"])
    }

    #[test]
    fn allow_returns_path_for_manifest_member() {
        let m = manifest();
        let p = m.allow("src/main.rs").unwrap();
        assert_eq!(p.as_str(), "src/main.rs");
    }

    #[test]
    fn allow_rejects_non_manifest_path() {
        let m = manifest();
        let err = m.allow("src/secret.rs").unwrap_err();
        // The error must not reveal the forbidden path or the allowed paths.
        let msg = err.to_string();
        assert!(!msg.contains("secret.rs"), "error leaks forbidden path");
        assert!(!msg.contains("src/main.rs"), "error leaks allowed paths");
    }

    #[test]
    fn allow_rejects_traversal() {
        let m = manifest();
        assert!(m.allow("../etc/passwd").is_err());
        assert!(m.allow("src/../../other").is_err());
    }

    #[test]
    fn allow_rejects_absolute() {
        let m = manifest();
        assert!(m.allow("/absolute").is_err());
    }

    #[test]
    fn len_and_iter() {
        let m = manifest();
        assert_eq!(m.len(), 2);
        let mut paths: Vec<&str> = m.iter().collect();
        paths.sort_unstable();
        assert_eq!(paths, ["README.md", "src/main.rs"]);
    }
}
