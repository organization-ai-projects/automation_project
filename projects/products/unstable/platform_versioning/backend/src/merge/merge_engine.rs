// projects/products/unstable/platform_versioning/backend/src/merge/merge.rs
use std::collections::{BTreeMap, BTreeSet};

use crate::diff::ContentClass;
use crate::errors::PvError;
use crate::ids::{BlobId, CommitId, ObjectId};
use crate::index::{Index, SafePath};
use crate::merge::{Conflict, ConflictKind, MergeResult};
use crate::objects::{Object, ObjectStore, TreeEntryKind};
use crate::pipeline::CommitBuilder;
use crate::refs_store::RefStore;

/// Merges two lines of history.
///
/// # Merge algorithm
/// This performs a three-way merge using the merge base (lowest common ancestor)
/// as the base version. The algorithm is deterministic: given the same inputs
/// and object store, the output is always identical.
///
/// # Binary handling policy
/// Binary files that differ between ours and theirs are always reported as
/// [`ConflictKind::Binary`] — they are never auto-merged.
pub struct Merge;

impl Merge {
    /// Attempts to merge `theirs` into `ours`, producing either a clean merge
    /// commit or a structured conflict report.
    ///
    /// On success, the merge commit is written to `object_store` and `ref_store`
    /// is updated to point to the new commit.
    pub fn perform(
        ours: &CommitId,
        theirs: &CommitId,
        author: &str,
        message: &str,
        timestamp_secs: u64,
        object_store: &ObjectStore,
        ref_store: &RefStore,
    ) -> Result<MergeResult, PvError> {
        let base = find_merge_base(ours, theirs, object_store)?;

        let base_files = base
            .as_ref()
            .map(|b| flatten_commit(b, object_store))
            .transpose()?
            .unwrap_or_default();
        let ours_files = flatten_commit(ours, object_store)?;
        let theirs_files = flatten_commit(theirs, object_store)?;

        let all_paths: BTreeSet<SafePath> = base_files
            .keys()
            .chain(ours_files.keys())
            .chain(theirs_files.keys())
            .cloned()
            .collect();

        let mut merged: BTreeMap<SafePath, BlobId> = BTreeMap::new();
        let mut conflicts: Vec<Conflict> = Vec::new();

        for path in all_paths {
            let base_blob = base_files.get(&path).cloned();
            let our_blob = ours_files.get(&path).cloned();
            let their_blob = theirs_files.get(&path).cloned();

            match (&base_blob, &our_blob, &their_blob) {
                // Both deleted (or both never had it).
                (_, None, None) => {}

                // Only ours has it (added by ours, or theirs deleted).
                (None, Some(o), None) => {
                    // Ours added, theirs didn't → keep ours.
                    merged.insert(path, o.clone());
                }

                // Only theirs has it (added by theirs, or ours deleted).
                (None, None, Some(t)) => {
                    // Theirs added, ours didn't → keep theirs.
                    merged.insert(path, t.clone());
                }

                // Ours deleted it; theirs still has it.
                (Some(b), None, Some(t)) => {
                    if b == t {
                        // Only ours deleted it, theirs unchanged → keep deletion.
                    } else {
                        // Theirs modified it while ours deleted → conflict.
                        conflicts.push(Conflict {
                            path,
                            kind: ConflictKind::DeleteModify,
                            base_blob,
                            ours_blob: None,
                            theirs_blob: Some(t.clone()),
                        });
                    }
                }

                // Theirs deleted it; ours still has it.
                (Some(b), Some(o), None) => {
                    if b == o {
                        // Only theirs deleted it, ours unchanged → keep deletion.
                    } else {
                        // Ours modified it while theirs deleted → conflict.
                        conflicts.push(Conflict {
                            path,
                            kind: ConflictKind::DeleteModify,
                            base_blob,
                            ours_blob: Some(o.clone()),
                            theirs_blob: None,
                        });
                    }
                }

                // Both ours and theirs have it.
                (base, Some(o), Some(t)) => {
                    if o == t {
                        // Identical in both → take either.
                        merged.insert(path, o.clone());
                    } else {
                        let b = base.as_ref();
                        if b == Some(t) {
                            // Only ours changed → take ours.
                            merged.insert(path, o.clone());
                        } else if b == Some(o) {
                            // Only theirs changed → take theirs.
                            merged.insert(path, t.clone());
                        } else {
                            // Both changed differently → conflict.
                            let our_class =
                                read_blob_class(o, object_store).unwrap_or(ContentClass::Binary);
                            let their_class =
                                read_blob_class(t, object_store).unwrap_or(ContentClass::Binary);

                            let conflict_kind = if our_class == ContentClass::Binary
                                || their_class == ContentClass::Binary
                            {
                                ConflictKind::Binary
                            } else if base.is_none() {
                                ConflictKind::AddAdd
                            } else {
                                ConflictKind::Content
                            };
                            conflicts.push(Conflict {
                                path,
                                kind: conflict_kind,
                                base_blob,
                                ours_blob: Some(o.clone()),
                                theirs_blob: Some(t.clone()),
                            });
                        }
                    }
                }
            }
        }

        if !conflicts.is_empty() {
            return Ok(MergeResult::Conflicted { conflicts });
        }

        // Build the merged commit.
        let mut index = Index::new();
        for (path, blob_id) in merged {
            index.add(path, blob_id);
        }

        if index.is_empty() {
            // Edge case: merge results in empty tree; still create the commit.
            // We need at least one entry for CommitBuilder — use a placeholder.
            // Actually this should just create an empty-tree commit.
            // CommitBuilder rejects empty index, so we handle this edge case.
            let commit = crate::objects::Commit::new(
                {
                    let snapshot = crate::pipeline::Snapshot::from_map(BTreeMap::new());
                    snapshot.write_trees(object_store)?
                },
                vec![ours.clone(), theirs.clone()],
                author.to_string(),
                message.to_string(),
                timestamp_secs,
            );
            let commit_id = commit.id.clone();
            object_store.write(Object::Commit(commit))?;
            return Ok(MergeResult::Clean { commit_id });
        }

        let result = CommitBuilder::new(author, message, timestamp_secs)
            .with_parent(theirs.clone())
            .commit(&index, object_store, ref_store)?;

        Ok(MergeResult::Clean {
            commit_id: result.commit_id,
        })
    }
}

/// Finds the merge base (lowest common ancestor) of two commits.
/// Returns `None` if the commits share no history.
fn find_merge_base(
    a: &CommitId,
    b: &CommitId,
    store: &ObjectStore,
) -> Result<Option<CommitId>, PvError> {
    // BFS from `a` to collect all ancestors.
    let ancestors_a = collect_ancestors(a, store)?;

    // BFS from `b`, returning the first commit also in `ancestors_a`.
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(b.clone());
    let mut visited = std::collections::HashSet::new();

    while let Some(current) = queue.pop_front() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());

        if ancestors_a.contains(&current) {
            return Ok(Some(current));
        }

        if let Ok(Object::Commit(c)) = store.read(current.as_object_id()) {
            for parent in c.parent_ids {
                queue.push_back(parent);
            }
        }
    }
    Ok(None)
}

fn collect_ancestors(
    start: &CommitId,
    store: &ObjectStore,
) -> Result<std::collections::HashSet<CommitId>, PvError> {
    let mut visited = std::collections::HashSet::new();
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start.clone());

    while let Some(current) = queue.pop_front() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());

        if let Ok(Object::Commit(c)) = store.read(current.as_object_id()) {
            for parent in c.parent_ids {
                queue.push_back(parent);
            }
        }
    }
    Ok(visited)
}

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
    use crate::objects::Blob;
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
        std::env::temp_dir().join(format!("pv_merge_{tag}_{pid}_{nanos}_{id}"))
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
    fn clean_merge_no_overlap() {
        let (obj, refs) = make_stores("clean");
        let _base = commit_files(&[("shared.txt", b"base")], 1, &obj, &refs);
        // Simulate two branches diverging from base.
        let ours = {
            let mut idx = Index::new();
            let blob = Blob::from_bytes(b"ours only".to_vec());
            idx.add("ours.txt".parse().unwrap(), blob.id.clone());
            obj.write(Object::Blob(blob)).unwrap();
            CommitBuilder::new("user", "ours", 2)
                .commit(&idx, &obj, &refs)
                .unwrap()
                .commit_id
        };

        // Create a parallel "theirs" by committing as a child of ours here.
        let theirs = commit_files(&[("theirs.txt", b"theirs only")], 3, &obj, &refs);

        let result = Merge::perform(&ours, &theirs, "merger", "merge", 4, &obj, &refs).unwrap();
        assert!(matches!(result, MergeResult::Clean { .. }));
    }

    #[test]
    fn conflicting_merge_detected() {
        let (obj, refs) = make_stores("conflict");
        let base = commit_files(&[("file.txt", b"base content")], 1, &obj, &refs);
        let ours = commit_files(&[("file.txt", b"ours modified")], 2, &obj, &refs);

        // Build `theirs` as a diverging branch from `base` (not from ours).
        // We create the commit object directly without going through the ref_store
        // to avoid picking up `ours` as a parent.
        let theirs = {
            let blob = Blob::from_bytes(b"theirs modified".to_vec());
            let their_blob_id = blob.id.clone();
            obj.write(Object::Blob(blob)).unwrap();

            // Build a snapshot with only file.txt
            let mut idx = Index::new();
            idx.add("file.txt".parse().unwrap(), their_blob_id);

            use crate::pipeline::Snapshot;
            use std::collections::BTreeMap;
            let map: BTreeMap<_, _> = idx.entries().map(|e| (e.path, e.blob_id)).collect();
            let snapshot = Snapshot::from_map(map);
            let tree_id = snapshot.write_trees(&obj).unwrap();

            let commit = crate::objects::Commit::new(
                tree_id,
                vec![base.clone()], // only base as parent — diverging from base
                "user".to_string(),
                "theirs".to_string(),
                3,
            );
            let theirs_id = commit.id.clone();
            obj.write(Object::Commit(commit)).unwrap();
            theirs_id
        };

        let result = Merge::perform(&ours, &theirs, "merger", "merge", 4, &obj, &refs).unwrap();
        assert!(matches!(result, MergeResult::Conflicted { .. }));
    }

    #[test]
    fn binary_conflict_detected() {
        let (obj, refs) = make_stores("binary");
        let base = commit_files(&[("img.png", b"base\x00binary")], 1, &obj, &refs);
        let ours = commit_files(&[("img.png", b"ours\x00binary")], 2, &obj, &refs);

        let theirs = {
            let blob = Blob::from_bytes(b"theirs\x00binary".to_vec());
            let their_blob_id = blob.id.clone();
            obj.write(Object::Blob(blob)).unwrap();

            let mut idx = Index::new();
            idx.add("img.png".parse().unwrap(), their_blob_id);

            use crate::pipeline::Snapshot;
            use std::collections::BTreeMap;
            let map: BTreeMap<_, _> = idx.entries().map(|e| (e.path, e.blob_id)).collect();
            let snapshot = Snapshot::from_map(map);
            let tree_id = snapshot.write_trees(&obj).unwrap();

            let commit = crate::objects::Commit::new(
                tree_id,
                vec![base.clone()],
                "user".to_string(),
                "theirs".to_string(),
                3,
            );
            let theirs_id = commit.id.clone();
            obj.write(Object::Commit(commit)).unwrap();
            theirs_id
        };

        let result = Merge::perform(&ours, &theirs, "merger", "merge", 4, &obj, &refs).unwrap();
        if let MergeResult::Conflicted { conflicts } = result {
            assert!(conflicts.iter().any(|c| c.kind == ConflictKind::Binary));
        } else {
            panic!("expected conflict");
        }
    }
}
