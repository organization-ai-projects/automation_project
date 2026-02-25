// projects/products/stable/platform_versioning/backend/src/diff/diff.rs
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::diffs::{ContentClass, DiffEntry, DiffKind};
use crate::errors::PvError;
use crate::ids::{BlobId, CommitId, ObjectId};
use crate::indexes::SafePath;
use crate::objects::{Object, ObjectStore, TreeEntryKind};

/// The diff between two revisions.
///
/// # Determinism
/// Entries are always sorted by path. Given the same two commit ids and the
/// same object store contents, the diff output is identical.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diff {
    /// The base commit id.
    pub from: CommitId,
    /// The target commit id.
    pub to: CommitId,
    /// The per-file diff entries, sorted by path.
    pub entries: Vec<DiffEntry>,
}

impl Diff {
    /// Computes the diff between `from` and `to`.
    pub fn compute(from: &CommitId, to: &CommitId, store: &ObjectStore) -> Result<Self, PvError> {
        let from_files = flatten_commit(from, store)?;
        let to_files = flatten_commit(to, store)?;

        let mut all_paths: std::collections::BTreeSet<SafePath> = BTreeSet::new();
        all_paths.extend(from_files.keys().cloned());
        all_paths.extend(to_files.keys().cloned());

        let mut entries = Vec::new();
        for path in all_paths {
            let from_blob = from_files.get(&path).cloned();
            let to_blob = to_files.get(&path).cloned();

            let kind = match (&from_blob, &to_blob) {
                (None, Some(_)) => DiffKind::Added,
                (Some(_), None) => DiffKind::Deleted,
                (Some(f), Some(t)) if f == t => continue, // unchanged — skip
                (Some(_), Some(_)) => DiffKind::Modified,
                (None, None) => continue,
            };

            let from_class = from_blob
                .as_ref()
                .and_then(|id| read_blob_class(id, store).ok());
            let to_class = to_blob
                .as_ref()
                .and_then(|id| read_blob_class(id, store).ok());

            entries.push(DiffEntry {
                path,
                kind,
                from_class,
                from_blob,
                to_class,
                to_blob,
            });
        }

        Ok(Self {
            from: from.clone(),
            to: to.clone(),
            entries,
        })
    }
}

use std::collections::BTreeSet;

fn flatten_commit(
    commit_id: &CommitId,
    store: &ObjectStore,
) -> Result<BTreeMap<SafePath, BlobId>, PvError> {
    let obj = store.read(commit_id.as_object_id())?;
    let tree_id = match obj {
        Object::Commit(ref c) => c.tree_id.as_object_id().clone(),
        _ => return Err(PvError::Internal(format!("{commit_id} is not a commit"))),
    };
    flatten_tree(&tree_id, "", store)
}

fn flatten_tree(
    tree_id: &ObjectId,
    prefix: &str,
    store: &ObjectStore,
) -> Result<BTreeMap<SafePath, BlobId>, PvError> {
    let obj = store.read(tree_id)?;
    let tree = match obj {
        Object::Tree(t) => t,
        _ => return Err(PvError::Internal(format!("{tree_id} is not a tree"))),
    };

    let mut result = BTreeMap::new();
    for entry in tree.entries {
        let path_str = if prefix.is_empty() {
            entry.name.clone()
        } else {
            format!("{}/{}", prefix, entry.name)
        };
        match entry.kind {
            TreeEntryKind::Blob => {
                if let Ok(safe) = path_str.parse::<SafePath>() {
                    result.insert(safe, BlobId::from(entry.id));
                }
            }
            TreeEntryKind::Tree => {
                let sub = flatten_tree(&entry.id, &path_str, store)?;
                result.extend(sub);
            }
        }
    }
    Ok(result)
}

fn read_blob_class(blob_id: &BlobId, store: &ObjectStore) -> Result<ContentClass, PvError> {
    let obj = store.read(blob_id.as_object_id())?;
    match obj {
        Object::Blob(b) => Ok(ContentClass::of(&b.content)),
        _ => Err(PvError::Internal(format!("{blob_id} is not a blob"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indexes::Index;
    use crate::objects::Blob;
    use crate::pipeline::CommitBuilder;
    use crate::refs_store::RefStore;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    fn unique_test_dir(tag: &str) -> std::path::PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let pid = std::process::id();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        std::env::temp_dir().join(format!("pv_diff_{tag}_{pid}_{nanos}_{id}"))
    }

    fn make_stores(tag: &str) -> (ObjectStore, RefStore) {
        let dir = unique_test_dir(tag);
        let obj = ObjectStore::open(&dir).unwrap();
        let refs = RefStore::open(&dir).unwrap();
        (obj, refs)
    }

    fn commit_files(
        files: &[(&str, &[u8])],
        ts: u64,
        obj: &ObjectStore,
        refs: &RefStore,
    ) -> CommitId {
        let mut idx = Index::new();
        for (path, content) in files {
            let blob = Blob::from_bytes(content.to_vec());
            idx.add(path.parse().unwrap(), blob.id.clone());
            obj.write(Object::Blob(blob)).unwrap();
        }
        CommitBuilder::new("user", "msg", ts)
            .commit(&idx, obj, refs)
            .unwrap()
            .commit_id
    }

    #[test]
    fn added_file_detected() {
        let (obj, refs) = make_stores("add");
        let c1 = commit_files(&[("a.txt", b"hello")], 1, &obj, &refs);
        let c2 = commit_files(&[("a.txt", b"hello"), ("b.txt", b"new")], 2, &obj, &refs);
        let diff = Diff::compute(&c1, &c2, &obj).unwrap();
        assert_eq!(diff.entries.len(), 1);
        assert_eq!(diff.entries[0].kind, DiffKind::Added);
        assert_eq!(diff.entries[0].path.as_str(), "b.txt");
    }

    #[test]
    fn deleted_file_detected() {
        let (obj, refs) = make_stores("del");
        let c1 = commit_files(&[("a.txt", b"hello"), ("b.txt", b"bye")], 1, &obj, &refs);
        let c2 = commit_files(&[("a.txt", b"hello")], 2, &obj, &refs);
        let diff = Diff::compute(&c1, &c2, &obj).unwrap();
        assert_eq!(diff.entries.len(), 1);
        assert_eq!(diff.entries[0].kind, DiffKind::Deleted);
    }

    #[test]
    fn modified_file_detected() {
        let (obj, refs) = make_stores("mod");
        let c1 = commit_files(&[("a.txt", b"v1")], 1, &obj, &refs);
        let c2 = commit_files(&[("a.txt", b"v2")], 2, &obj, &refs);
        let diff = Diff::compute(&c1, &c2, &obj).unwrap();
        assert_eq!(diff.entries.len(), 1);
        assert_eq!(diff.entries[0].kind, DiffKind::Modified);
    }

    #[test]
    fn identical_revisions_produce_empty_diff() {
        let (obj, refs) = make_stores("same");
        let c1 = commit_files(&[("a.txt", b"same")], 1, &obj, &refs);
        let c2 = commit_files(&[("a.txt", b"same")], 2, &obj, &refs);
        let diff = Diff::compute(&c1, &c2, &obj).unwrap();
        assert!(diff.entries.is_empty());
    }

    #[test]
    fn diff_output_is_deterministic() {
        let (obj, refs) = make_stores("det");
        let c1 = commit_files(&[("f.txt", b"old")], 1, &obj, &refs);
        let c2 = commit_files(&[("f.txt", b"new")], 2, &obj, &refs);
        let d1 = Diff::compute(&c1, &c2, &obj).unwrap();
        let d2 = Diff::compute(&c1, &c2, &obj).unwrap();
        assert_eq!(d1, d2);
    }
}
