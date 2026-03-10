// projects/products/stable/platform_versioning/backend/src/merge/merge.rs
use std::collections::{BTreeMap, BTreeSet};

use crate::diffs::ContentClass;
use crate::errors::PvError;
use crate::ids::{BlobId, CommitId, ObjectId};
use crate::indexes::{Index, SafePath};
use crate::merges::{Conflict, ConflictKind, MergeResult};
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
