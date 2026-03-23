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
