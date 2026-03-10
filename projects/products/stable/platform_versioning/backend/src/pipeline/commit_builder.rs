// projects/products/stable/platform_versioning/backend/src/pipeline/commit_builder.rs
use std::collections::BTreeMap;

use crate::errors::PvError;
use crate::ids::BlobId;
use crate::indexes::{Index, SafePath};
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
