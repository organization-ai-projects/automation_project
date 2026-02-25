// projects/products/stable/platform_versioning/backend/src/index/index.rs
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::errors::PvError;
use crate::ids::BlobId;
use crate::indexes::{IndexEntry, SafePath};

/// Current encoding format version for the index.
const INDEX_FORMAT_VERSION: u8 = 1;

/// The staging index: a versioned, ordered map from safe path to blob id.
///
/// # Format version
/// Version 1. The `version` field must be checked on deserialization; an
/// unrecognized version returns [`PvError::UnsupportedVersion`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Index {
    /// Format version for forward-compatibility detection.
    pub version: u8,
    /// Entries keyed by path, sorted for deterministic iteration.
    entries: BTreeMap<SafePath, BlobId>,
}

impl Index {
    /// Creates an empty index.
    pub fn new() -> Self {
        Self {
            version: INDEX_FORMAT_VERSION,
            entries: BTreeMap::new(),
        }
    }

    /// Validates the version field.
    pub fn check_version(&self) -> Result<(), PvError> {
        if self.version != INDEX_FORMAT_VERSION {
            return Err(PvError::UnsupportedVersion(format!(
                "index version {} is not supported (expected {})",
                self.version, INDEX_FORMAT_VERSION
            )));
        }
        Ok(())
    }

    /// Adds or updates an entry. Returns the previous blob id if the path existed.
    pub fn add(&mut self, path: SafePath, blob_id: BlobId) -> Option<BlobId> {
        self.entries.insert(path, blob_id)
    }

    /// Removes an entry. Returns the removed blob id, or `None` if not present.
    pub fn remove(&mut self, path: &SafePath) -> Option<BlobId> {
        self.entries.remove(path)
    }

    /// Returns the blob id for the given path, if staged.
    pub fn get(&self, path: &SafePath) -> Option<&BlobId> {
        self.entries.get(path)
    }

    /// Returns all entries in deterministic (sorted) order.
    pub fn entries(&self) -> impl Iterator<Item = IndexEntry> + '_ {
        self.entries.iter().map(|(path, blob_id)| IndexEntry {
            path: path.clone(),
            blob_id: blob_id.clone(),
        })
    }

    /// Returns the number of staged entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if no entries are staged.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for Index {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::ObjectId;

    fn dummy_blob_id(byte: u8) -> BlobId {
        BlobId::from(ObjectId::from_bytes(&[byte; 32]))
    }

    fn path(s: &str) -> SafePath {
        s.parse().unwrap()
    }

    #[test]
    fn add_and_get() {
        let mut idx = Index::new();
        let bid = dummy_blob_id(1);
        idx.add(path("src/main.rs"), bid.clone());
        assert_eq!(idx.get(&path("src/main.rs")), Some(&bid));
    }

    #[test]
    fn remove_existing() {
        let mut idx = Index::new();
        idx.add(path("foo.txt"), dummy_blob_id(1));
        let removed = idx.remove(&path("foo.txt"));
        assert!(removed.is_some());
        assert_eq!(idx.len(), 0);
    }

    #[test]
    fn remove_missing_returns_none() {
        let mut idx = Index::new();
        assert_eq!(idx.remove(&path("nonexistent.txt")), None);
    }

    #[test]
    fn entries_are_sorted() {
        let mut idx = Index::new();
        idx.add(path("z.txt"), dummy_blob_id(1));
        idx.add(path("a.txt"), dummy_blob_id(2));
        let names: Vec<_> = idx.entries().map(|e| e.path.to_string()).collect();
        assert_eq!(names, vec!["a.txt", "z.txt"]);
    }

    #[test]
    fn version_check_passes_for_current() {
        let idx = Index::new();
        assert!(idx.check_version().is_ok());
    }

    #[test]
    fn version_check_fails_for_future() {
        let mut idx = Index::new();
        idx.version = 255;
        assert!(idx.check_version().is_err());
    }

    #[test]
    fn unsafe_path_rejected_by_safe_path() {
        assert!("../etc/passwd".parse::<SafePath>().is_err());
    }
}
