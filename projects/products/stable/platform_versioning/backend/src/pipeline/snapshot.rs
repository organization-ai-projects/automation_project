// projects/products/stable/platform_versioning/backend/src/pipeline/snapshot.rs
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::errors::PvError;
use crate::ids::{BlobId, TreeId};
use crate::indexes::SafePath;
use crate::objects::{Object, ObjectStore, Tree, TreeEntry, TreeEntryKind};
use crate::pipeline::SnapshotEntry;

/// A flat, sorted snapshot of all files in a working tree at a point in time.
///
/// A `Snapshot` is built from an [`crate::indexes::Index`] and can materialize
/// the nested [`Tree`] objects required by the commit pipeline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    /// Entries in deterministic (sorted by path) order.
    pub entries: Vec<SnapshotEntry>,
}

impl Snapshot {
    /// Builds a `Snapshot` from a sorted map of path → blob_id.
    pub fn from_map(map: BTreeMap<SafePath, BlobId>) -> Self {
        let entries = map
            .into_iter()
            .map(|(path, blob_id)| SnapshotEntry { path, blob_id })
            .collect();
        Self { entries }
    }

    /// Materializes the nested tree objects into `store` and returns the root [`TreeId`].
    ///
    /// # Determinism
    /// Given identical entries, the resulting root `TreeId` is always identical
    /// because `Tree::from_entries` sorts entries and hashing is deterministic.
    pub fn write_trees(&self, store: &ObjectStore) -> Result<TreeId, PvError> {
        // Build a nested directory map: dir_path -> [(name, kind, id)]
        // We use a BTreeMap so parent dirs are processed before children.
        let mut dir_entries: BTreeMap<String, Vec<TreeEntry>> = BTreeMap::new();
        dir_entries.insert(String::new(), vec![]);

        for entry in &self.entries {
            let path = entry.path.as_str();
            let blob_id = entry.blob_id.as_object_id().clone();
            let (dir, name) = split_path(path);

            // Ensure all intermediate directories exist.
            let mut prefix = String::new();
            for component in dir.split('/') {
                if component.is_empty() {
                    continue;
                }
                if !prefix.is_empty() {
                    prefix.push('/');
                }
                prefix.push_str(component);
                dir_entries.entry(prefix.clone()).or_default();
            }

            let tree_entry = TreeEntry {
                name: name.to_string(),
                kind: TreeEntryKind::Blob,
                id: blob_id,
            };
            dir_entries
                .entry(dir.to_string())
                .or_default()
                .push(tree_entry);
        }

        // Process directories deepest-first (longest path first) so that
        // subtree ids are available when building parent trees.
        let mut dirs: Vec<String> = dir_entries.keys().cloned().collect();
        dirs.sort_by(|a, b| b.len().cmp(&a.len()).then(b.cmp(a)));

        let mut subtree_ids: BTreeMap<String, TreeId> = BTreeMap::new();

        for dir in &dirs {
            let mut entries = dir_entries.remove(dir).unwrap_or_default();

            // Add subtree entries for immediate children.
            let dir_prefix = if dir.is_empty() {
                String::new()
            } else {
                format!("{}/", dir)
            };
            for (sub_path, sub_id) in &subtree_ids {
                if let Some(rest) = sub_path.strip_prefix(&dir_prefix)
                    && !rest.contains('/')
                {
                    entries.push(TreeEntry {
                        name: rest.to_string(),
                        kind: TreeEntryKind::Tree,
                        id: sub_id.as_object_id().clone(),
                    });
                }
            }

            let tree = Tree::from_entries(entries);
            let tree_id = tree.id.clone();
            store.write(Object::Tree(tree))?;
            subtree_ids.insert(dir.clone(), tree_id);
        }

        subtree_ids.get("").cloned().ok_or_else(|| {
            PvError::Internal("root tree not found after snapshot write".to_string())
        })
    }
}

fn split_path(path: &str) -> (&str, &str) {
    if let Some(pos) = path.rfind('/') {
        (&path[..pos], &path[pos + 1..])
    } else {
        ("", path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    fn unique_test_dir() -> std::path::PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let pid = std::process::id();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        std::env::temp_dir().join(format!("pv_snapshot_{pid}_{nanos}_{id}"))
    }

    fn dummy_blob_id(byte: u8) -> BlobId {
        BlobId::from(crate::ids::ObjectId::from_bytes(&[byte; 32]))
    }

    #[test]
    fn empty_snapshot_produces_empty_root_tree() {
        let dir = unique_test_dir();
        let store = ObjectStore::open(&dir).unwrap();
        let snapshot = Snapshot::from_map(BTreeMap::new());
        let root_id = snapshot.write_trees(&store).unwrap();
        let obj = store.read(root_id.as_object_id()).unwrap();
        match obj {
            Object::Tree(t) => assert!(t.entries.is_empty()),
            _ => panic!("expected tree"),
        }
    }

    #[test]
    fn flat_snapshot_produces_correct_tree() {
        let dir = unique_test_dir();
        let store = ObjectStore::open(&dir).unwrap();
        let mut map = BTreeMap::new();
        map.insert("a.txt".parse().unwrap(), dummy_blob_id(1));
        map.insert("b.txt".parse().unwrap(), dummy_blob_id(2));
        let snapshot = Snapshot::from_map(map);
        let root_id = snapshot.write_trees(&store).unwrap();
        let obj = store.read(root_id.as_object_id()).unwrap();
        match obj {
            Object::Tree(t) => assert_eq!(t.entries.len(), 2),
            _ => panic!("expected tree"),
        }
    }

    #[test]
    fn same_snapshot_produces_same_tree_id() {
        let dir1 = unique_test_dir();
        let dir2 = unique_test_dir();
        let store1 = ObjectStore::open(&dir1).unwrap();
        let store2 = ObjectStore::open(&dir2).unwrap();
        let mut map = BTreeMap::new();
        map.insert("readme.md".parse().unwrap(), dummy_blob_id(5));
        let s1 = Snapshot::from_map(map.clone());
        let s2 = Snapshot::from_map(map);
        let id1 = s1.write_trees(&store1).unwrap();
        let id2 = s2.write_trees(&store2).unwrap();
        assert_eq!(id1, id2);
    }
}
