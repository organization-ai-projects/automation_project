// projects/products/stable/platform_versioning/backend/src/slices/slice_definition.rs
use serde::{Deserialize, Serialize};

use crate::errors::PvError;
use crate::indexes::SafePath;

/// A validated allowlist of repository paths defining a work scope.
///
/// All paths are validated with the same safety rules as [`SafePath`]: no
/// traversal components (`..`), no absolute paths, no null bytes, and no
/// backslashes. An empty allowlist is permitted and means the issue has no
/// associated file scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SliceDefinition {
    paths: Vec<SafePath>,
}

impl SliceDefinition {
    /// Creates a [`SliceDefinition`] from a list of raw path strings.
    ///
    /// Every path is validated; the first invalid path causes the whole call
    /// to fail.
    pub fn from_paths(raw: Vec<String>) -> Result<Self, PvError> {
        let mut paths = Vec::with_capacity(raw.len());
        for s in raw {
            let safe = s
                .parse::<SafePath>()
                .map_err(|e| PvError::SliceBuildFailed(format!("invalid slice path '{s}': {e}")))?;
            paths.push(safe);
        }
        paths.sort();
        paths.dedup();
        Ok(Self { paths })
    }

    /// Returns the validated path allowlist.
    pub fn paths(&self) -> &[SafePath] {
        &self.paths
    }

    /// Returns `true` if `path` is within this slice definition.
    ///
    /// A path is included if it exactly matches an entry or is a descendant of
    /// an allowed directory prefix.
    pub fn contains(&self, path: &SafePath) -> bool {
        let target = path.as_str();
        self.paths.iter().any(|allowed| {
            let prefix = allowed.as_str();
            target == prefix || target.starts_with(&format!("{prefix}/"))
        })
    }
}
