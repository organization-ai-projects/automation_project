// projects/products/stable/platform_versioning/backend/src/repos/repo.rs
use serde::{Deserialize, Serialize};

use crate::ids::RepoId;
use crate::objects::ObjectStore;
use crate::refs_store::RefStore;
use crate::repos::RepoMetadata;

/// A repository: metadata + object store + ref store.
#[derive(Clone, Serialize, Deserialize)]
pub struct Repo {
    /// Repository metadata.
    pub metadata: RepoMetadata,
    /// Content-addressed object store for this repository.
    pub objects: ObjectStore,
    /// Mutable ref store for this repository.
    pub refs: RefStore,
}

impl Repo {
    /// Returns the repository id.
    pub fn id(&self) -> &RepoId {
        &self.metadata.id
    }
}
