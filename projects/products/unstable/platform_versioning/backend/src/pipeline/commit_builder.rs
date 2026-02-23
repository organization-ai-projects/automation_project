// projects/products/unstable/platform_versioning/backend/src/pipeline/commit_builder.rs
use std::collections::BTreeMap;

use crate::errors::PvError;
use crate::ids::BlobId;
use crate::index::{Index, SafePath};
use crate::objects::{Commit, Object, ObjectStore};
use crate::pipeline::{CommitResult, Snapshot};
use crate::refs_store::{HeadState, RefStore, RefTarget};

/// Builds a commit from a staged index.
///
/// # Determinism
/// Given the same index contents, author, message, and timestamp, the resulting
/// commit id is always identical. Callers that need reproducible ids must supply
/// a fixed `timestamp_secs`.
pub struct CommitBuilder {
    author: String,
    message: String,
    timestamp_secs: u64,
    extra_parents: Vec<crate::ids::CommitId>,
}

impl CommitBuilder {
    /// Creates a new builder with the given author, message, and timestamp.
    pub fn new(author: impl Into<String>, message: impl Into<String>, timestamp_secs: u64) -> Self {
        Self {
            author: author.into(),
            message: message.into(),
            timestamp_secs,
            extra_parents: vec![],
        }
    }

    /// Adds an extra parent commit (for merge commits).
    pub fn with_parent(mut self, parent: crate::ids::CommitId) -> Self {
        self.extra_parents.push(parent);
        self
    }

    /// Executes the commit pipeline:
    /// 1. Validates the index (must not be empty).
    /// 2. Builds the snapshot from the index entries.
    /// 3. Materializes tree objects into `object_store`.
    /// 4. Creates the commit object and stores it.
    /// 5. Updates the HEAD ref.
    ///
    /// Returns a [`CommitResult`] containing the new commit id.
    pub fn commit(
        self,
        index: &Index,
        object_store: &ObjectStore,
        ref_store: &RefStore,
    ) -> Result<CommitResult, PvError> {
        index.check_version()?;

        if index.is_empty() {
            return Err(PvError::Internal(
                "cannot commit: no changes staged".to_string(),
            ));
        }

        // Build snapshot from index.
        let map: BTreeMap<SafePath, BlobId> =
            index.entries().map(|e| (e.path, e.blob_id)).collect();
        let snapshot = Snapshot::from_map(map);
        let root_tree_id = snapshot.write_trees(object_store)?;

        // Resolve parent commits.
        let mut parent_ids = self.extra_parents;
        let head = ref_store.read_head()?;
        if let HeadState::Branch(ref branch) = head
            && let Ok(target) = ref_store.read_ref(branch)
        {
            parent_ids.insert(0, target.commit_id().clone());
        }

        // Create and store the commit.
        let commit = Commit::new(
            root_tree_id,
            parent_ids,
            self.author,
            self.message,
            self.timestamp_secs,
        );
        let commit_id = commit.id.clone();
        object_store.write(Object::Commit(commit))?;

        // Update HEAD ref.
        let updated_ref = match &head {
            HeadState::Branch(branch) | HeadState::Unborn(branch) => {
                let branch = branch.clone();
                ref_store.write_ref(&branch, &RefTarget::Commit(commit_id.clone()), true, None)?;
                ref_store.write_head(&HeadState::Branch(branch.clone()))?;
                Some(branch.to_string())
            }
            HeadState::Detached(_) => {
                ref_store.write_head(&HeadState::Detached(commit_id.to_string()))?;
                None
            }
        };

        Ok(CommitResult {
            commit_id,
            updated_ref,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::Blob;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    fn unique_test_dir(tag: &str) -> std::path::PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let pid = std::process::id();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        std::env::temp_dir().join(format!("pv_commit_{tag}_{pid}_{nanos}_{id}"))
    }

    fn make_stores(tag: &str) -> (ObjectStore, RefStore) {
        let dir = unique_test_dir(tag);
        let obj = ObjectStore::open(&dir).unwrap();
        let refs = RefStore::open(&dir).unwrap();
        (obj, refs)
    }

    fn stage_blob(index: &mut Index, path: &str, content: &[u8]) {
        let blob = Blob::from_bytes(content.to_vec());
        let blob_id = blob.id.clone();
        // Store the blob somewhere (not needed for commit test, but realistic).
        let _ = blob;
        index.add(path.parse().unwrap(), blob_id);
    }

    #[test]
    fn initial_commit_succeeds() {
        let (obj_store, ref_store) = make_stores("initial");
        let mut index = Index::new();
        stage_blob(&mut index, "README.md", b"# Hello");
        let builder = CommitBuilder::new("Alice", "Initial commit", 1_000_000);
        let result = builder.commit(&index, &obj_store, &ref_store).unwrap();
        assert!(result.updated_ref.is_some());
    }

    #[test]
    fn empty_index_is_rejected() {
        let (obj_store, ref_store) = make_stores("empty");
        let index = Index::new();
        let builder = CommitBuilder::new("Alice", "noop", 0);
        let result = builder.commit(&index, &obj_store, &ref_store);
        assert!(matches!(result, Err(PvError::Internal(_))));
    }

    #[test]
    fn second_commit_has_parent() {
        let (obj_store, ref_store) = make_stores("second");

        // First commit.
        let mut idx1 = Index::new();
        stage_blob(&mut idx1, "a.txt", b"first");
        let r1 = CommitBuilder::new("Bob", "first", 1_000)
            .commit(&idx1, &obj_store, &ref_store)
            .unwrap();

        // Second commit.
        let mut idx2 = Index::new();
        stage_blob(&mut idx2, "b.txt", b"second");
        let r2 = CommitBuilder::new("Bob", "second", 2_000)
            .commit(&idx2, &obj_store, &ref_store)
            .unwrap();

        // Verify parent relationship.
        use crate::objects::Object;
        let commit = obj_store.read(r2.commit_id.as_object_id()).unwrap();
        if let Object::Commit(c) = commit {
            assert_eq!(c.parent_ids, vec![r1.commit_id]);
        } else {
            panic!("expected commit object");
        }
    }

    #[test]
    fn same_inputs_produce_same_commit_id() {
        let (obj1, ref1) = make_stores("det1");
        let (obj2, ref2) = make_stores("det2");
        let mut idx1 = Index::new();
        let mut idx2 = Index::new();
        stage_blob(&mut idx1, "f.txt", b"content");
        stage_blob(&mut idx2, "f.txt", b"content");
        let r1 = CommitBuilder::new("X", "msg", 42)
            .commit(&idx1, &obj1, &ref1)
            .unwrap();
        let r2 = CommitBuilder::new("X", "msg", 42)
            .commit(&idx2, &obj2, &ref2)
            .unwrap();
        assert_eq!(r1.commit_id, r2.commit_id);
    }
}
