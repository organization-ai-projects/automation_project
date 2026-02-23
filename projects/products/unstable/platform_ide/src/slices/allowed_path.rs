// projects/products/unstable/platform_ide/src/slices/allowed_path.rs
use serde::{Deserialize, Serialize};

/// A validated file path that has been confirmed to exist within the slice
/// manifest for the current issue.
///
/// `AllowedPath` can only be constructed through [`super::SliceManifest::allow`],
/// which checks membership before granting the typed path. This prevents any
/// code path from accidentally operating on a forbidden file.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AllowedPath(String);

impl AllowedPath {
    /// Creates an `AllowedPath` that has already been validated by the manifest.
    ///
    /// Only [`super::SliceManifest`] should call this constructor in production
    /// code; it is also accessible in test code for constructing test fixtures.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn new_validated(path: String) -> Self {
        Self(path)
    }

    /// Returns the path string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AllowedPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Validates that a raw path string does not contain directory traversal
/// components or absolute path indicators before the manifest membership
/// check is performed.
pub(super) fn is_safe_path(raw: &str) -> bool {
    if raw.is_empty() {
        return false;
    }
    // Reject absolute paths and traversal sequences.
    if raw.starts_with('/') || raw.starts_with('\\') {
        return false;
    }
    for component in raw.split(['/', '\\']) {
        if component == ".." || component == "." {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_path_accepts_normal_paths() {
        assert!(is_safe_path("src/main.rs"));
        assert!(is_safe_path("README.md"));
        assert!(is_safe_path("a/b/c/d.txt"));
    }

    #[test]
    fn safe_path_rejects_traversal() {
        assert!(!is_safe_path("../etc/passwd"));
        assert!(!is_safe_path("src/../../secret"));
        assert!(!is_safe_path("/absolute/path"));
        assert!(!is_safe_path(""));
    }

    #[test]
    fn allowed_path_display() {
        let p = AllowedPath::new_validated("src/lib.rs".to_string());
        assert_eq!(p.to_string(), "src/lib.rs");
        assert_eq!(p.as_str(), "src/lib.rs");
    }

    #[test]
    fn allowed_path_equality() {
        let a = AllowedPath::new_validated("src/lib.rs".to_string());
        let b = AllowedPath::new_validated("src/lib.rs".to_string());
        assert_eq!(a, b);
    }
}
