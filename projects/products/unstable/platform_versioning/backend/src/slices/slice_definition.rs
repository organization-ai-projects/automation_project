// projects/products/unstable/platform_versioning/backend/src/slices/slice_definition.rs
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
            let safe = s.parse::<SafePath>().map_err(|e| {
                PvError::SliceBuildFailed(format!("invalid slice path '{s}': {e}"))
            })?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_definition_is_valid() {
        let def = SliceDefinition::from_paths(vec![]).unwrap();
        assert!(def.paths().is_empty());
    }

    #[test]
    fn valid_paths_are_accepted() {
        let def =
            SliceDefinition::from_paths(vec!["src".to_string(), "docs/guide.md".to_string()])
                .unwrap();
        assert_eq!(def.paths().len(), 2);
    }

    #[test]
    fn traversal_path_is_rejected() {
        assert!(SliceDefinition::from_paths(vec!["../etc/passwd".to_string()]).is_err());
    }

    #[test]
    fn absolute_path_is_rejected() {
        assert!(SliceDefinition::from_paths(vec!["/etc/passwd".to_string()]).is_err());
    }

    #[test]
    fn contains_exact_match() {
        let def = SliceDefinition::from_paths(vec!["src/main.rs".to_string()]).unwrap();
        assert!(def.contains(&"src/main.rs".parse().unwrap()));
    }

    #[test]
    fn contains_child_of_allowed_dir() {
        let def = SliceDefinition::from_paths(vec!["src".to_string()]).unwrap();
        assert!(def.contains(&"src/lib.rs".parse().unwrap()));
        assert!(def.contains(&"src/sub/mod.rs".parse().unwrap()));
    }

    #[test]
    fn does_not_contain_sibling_dir() {
        let def = SliceDefinition::from_paths(vec!["src".to_string()]).unwrap();
        assert!(!def.contains(&"tests/integration.rs".parse().unwrap()));
    }

    #[test]
    fn paths_are_deduplicated_and_sorted() {
        let def = SliceDefinition::from_paths(vec![
            "b/file.rs".to_string(),
            "a/file.rs".to_string(),
            "a/file.rs".to_string(),
        ])
        .unwrap();
        assert_eq!(def.paths().len(), 2);
        assert_eq!(def.paths()[0].as_str(), "a/file.rs");
    }
}
