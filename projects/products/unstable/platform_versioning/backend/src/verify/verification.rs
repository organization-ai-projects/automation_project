// projects/products/unstable/platform_versioning/backend/src/verify/verification.rs
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::index::Index;
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
        std::env::temp_dir().join(format!("pv_verify_{tag}_{pid}_{nanos}_{id}"))
    }

    fn make_stores(tag: &str) -> (ObjectStore, RefStore) {
        let dir = unique_test_dir(tag);
        let obj = ObjectStore::open(&dir).unwrap();
        let refs = RefStore::open(&dir).unwrap();
        (obj, refs)
    }

    #[test]
    fn healthy_repo_passes() {
        let (obj, refs) = make_stores("healthy");
        let blob = Blob::from_bytes(b"hello".to_vec());
        let mut idx = Index::new();
        idx.add("hello.txt".parse().unwrap(), blob.id.clone());
        obj.write(Object::Blob(blob)).unwrap();
        CommitBuilder::new("user", "init", 1)
            .commit(&idx, &obj, &refs)
            .unwrap();
        let report = Verification::run(&obj, &refs).unwrap();
        assert!(report.is_healthy(), "{:?}", report.issues);
    }

    #[test]
    fn dangling_ref_detected() {
        let (obj, refs) = make_stores("dangle");
        // Write a ref pointing to a nonexistent commit.
        let fake_id = crate::ids::CommitId::from(ObjectId::from_bytes(&[0xffu8; 32]));
        let ref_name: crate::refs_store::RefName = "heads/main".parse().unwrap();
        refs.write_ref(
            &ref_name,
            &crate::refs_store::RefTarget::Commit(fake_id),
            true,
            None,
        )
        .unwrap();
        let report = Verification::run(&obj, &refs).unwrap();
        assert!(!report.is_healthy());
        assert!(
            report
                .issues
                .iter()
                .any(|i| matches!(i, IntegrityIssue::DanglingRef { .. }))
        );
    }

    #[test]
    fn empty_repo_is_healthy() {
        let (obj, refs) = make_stores("empty");
        let report = Verification::run(&obj, &refs).unwrap();
        assert!(report.is_healthy());
    }
}
