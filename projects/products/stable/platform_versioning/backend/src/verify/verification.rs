// projects/products/stable/platform_versioning/backend/src/verify/verification.rs
use std::collections::{HashSet, VecDeque};

use crate::errors::PvError;
use crate::ids::ObjectId;
use crate::objects::{Object, ObjectStore, TreeEntryKind};
use crate::refs_store::RefStore;
use crate::verify::{IntegrityIssue, IntegrityReport};

/// Repository integrity verifier.
///
/// Checks:
/// - All refs point to existing commits.
/// - All reachable objects (commits, trees, blobs) exist and pass integrity
///   validation.
/// - Trees reference only existing objects.
pub struct Verification;

impl Verification {
    /// Runs a full integrity check and returns a report.
    pub fn run(
        object_store: &ObjectStore,
        ref_store: &RefStore,
    ) -> Result<IntegrityReport, PvError> {
        let mut issues = Vec::new();
        let mut reachable: HashSet<ObjectId> = HashSet::new();

        let refs = ref_store.list_refs()?;
        let refs_checked = refs.len();

        // 1. Verify all refs resolve to existing commits.
        let mut queue: VecDeque<ObjectId> = VecDeque::new();
        for (ref_name, target) in &refs {
            let commit_id = target.commit_id().as_object_id().clone();
            if !object_store.exists(&commit_id) {
                issues.push(IntegrityIssue::DanglingRef {
                    ref_name: ref_name.to_string(),
                    target: commit_id.to_string(),
                });
            } else {
                queue.push_back(commit_id);
            }
        }

        // 2. Traverse all reachable objects.
        let mut objects_checked = 0usize;
        while let Some(id) = queue.pop_front() {
            if reachable.contains(&id) {
                continue;
            }
            reachable.insert(id.clone());
            objects_checked += 1;

            match object_store.read(&id) {
                Ok(obj) => {
                    if !obj.verify() {
                        issues.push(IntegrityIssue::CorruptObject {
                            object_id: id.to_string(),
                        });
                    } else {
                        match obj {
                            Object::Commit(c) => {
                                let tree_obj_id = c.tree_id.as_object_id().clone();
                                if !object_store.exists(&tree_obj_id) {
                                    issues.push(IntegrityIssue::MissingTree {
                                        commit_id: id.to_string(),
                                        tree_id: tree_obj_id.to_string(),
                                    });
                                } else {
                                    queue.push_back(tree_obj_id);
                                }
                                for parent in c.parent_ids {
                                    queue.push_back(parent.as_object_id().clone());
                                }
                            }
                            Object::Tree(t) => {
                                for entry in &t.entries {
                                    if !object_store.exists(&entry.id) {
                                        issues.push(IntegrityIssue::MissingObject {
                                            tree_id: id.to_string(),
                                            entry_name: entry.name.clone(),
                                        });
                                    } else if entry.kind == TreeEntryKind::Tree
                                        || entry.kind == TreeEntryKind::Blob
                                    {
                                        queue.push_back(entry.id.clone());
                                    }
                                }
                            }
                            Object::Blob(_) => {}
                        }
                    }
                }
                Err(_) => {
                    issues.push(IntegrityIssue::CorruptObject {
                        object_id: id.to_string(),
                    });
                }
            }
        }

        Ok(IntegrityReport {
            issues,
            objects_checked,
            refs_checked,
        })
    }
}
