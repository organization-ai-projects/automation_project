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
