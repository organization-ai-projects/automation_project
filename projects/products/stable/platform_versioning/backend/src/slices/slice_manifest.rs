// projects/products/stable/platform_versioning/backend/src/slices/slice_manifest.rs
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
