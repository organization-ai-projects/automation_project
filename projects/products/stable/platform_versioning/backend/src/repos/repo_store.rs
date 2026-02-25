// projects/products/stable/platform_versioning/backend/src/repos/repo_store.rs
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use crate::errors::PvError;
use crate::ids::RepoId;
use crate::objects::ObjectStore;
use crate::refs_store::RefStore;
use crate::repos::{Repo, RepoMetadata};

/// Registry of all repositories on this server.
///
/// # On-disk layout
/// ```text
/// <root>/
///   repos/
///     <repo_id>/
///       metadata.json
///       objects/   (managed by ObjectStore)
///       refs/      (managed by RefStore)
/// ```
#[derive(Clone)]
pub struct RepoStore {
    root: PathBuf,
    cache: Arc<RwLock<HashMap<RepoId, Repo>>>,
}

impl RepoStore {
    /// Opens (or creates) the repo store at `root`.
    pub fn open(root: impl AsRef<Path>) -> Result<Self, PvError> {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(root.join("repos"))
            .map_err(|e| PvError::AtomicWriteFailed(format!("create repos dir: {e}")))?;
        Ok(Self {
            root,
            cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Creates a new repository.
    pub fn create(
        &self,
        id: RepoId,
        name: String,
        description: Option<String>,
        now_secs: u64,
    ) -> Result<Repo, PvError> {
        if self.exists(&id) {
            return Err(PvError::Internal(format!(
                "repository '{}' already exists",
                id
            )));
        }

        let repo_dir = self.repo_dir(&id);
        fs::create_dir_all(&repo_dir)
            .map_err(|e| PvError::AtomicWriteFailed(format!("create repo dir: {e}")))?;

        let metadata = RepoMetadata {
            id: id.clone(),
            name,
            description,
            created_at: now_secs,
            updated_at: now_secs,
        };

        let meta_path = repo_dir.join("metadata.json");
        let meta_bytes = common_json::to_bytes(&metadata)
            .map_err(|e| PvError::Internal(format!("serialize metadata: {e}")))?;
        fs::write(&meta_path, &meta_bytes)
            .map_err(|e| PvError::AtomicWriteFailed(format!("write metadata: {e}")))?;

        let objects = ObjectStore::open(&repo_dir)?;
        let refs = RefStore::open(&repo_dir)?;

        let repo = Repo {
            metadata,
            objects,
            refs,
        };

        if let Ok(mut cache) = self.cache.write() {
            cache.insert(id, repo.clone());
        }

        Ok(repo)
    }

    /// Reads a repository by id.
    pub fn get(&self, id: &RepoId) -> Result<Repo, PvError> {
        if let Ok(cache) = self.cache.read()
            && let Some(repo) = cache.get(id)
        {
            return Ok(repo.clone());
        }

        let repo_dir = self.repo_dir(id);
        if !repo_dir.exists() {
            return Err(PvError::RepoNotFound(id.to_string()));
        }

        let meta_bytes = fs::read(repo_dir.join("metadata.json"))
            .map_err(|e| PvError::Internal(format!("read metadata: {e}")))?;
        let metadata: RepoMetadata = common_json::from_slice(&meta_bytes)
            .map_err(|e| PvError::Internal(format!("parse metadata: {e}")))?;

        let objects = ObjectStore::open(&repo_dir)?;
        let refs = RefStore::open(&repo_dir)?;

        let repo = Repo {
            metadata,
            objects,
            refs,
        };

        if let Ok(mut cache) = self.cache.write() {
            cache.insert(id.clone(), repo.clone());
        }

        Ok(repo)
    }

    /// Updates the mutable metadata fields of a repository.
    pub fn update_metadata(
        &self,
        id: &RepoId,
        name: Option<String>,
        description: Option<Option<String>>,
        now_secs: u64,
    ) -> Result<RepoMetadata, PvError> {
        let mut repo = self.get(id)?;
        if let Some(n) = name {
            repo.metadata.name = n;
        }
        if let Some(d) = description {
            repo.metadata.description = d;
        }
        repo.metadata.updated_at = now_secs;

        let meta_path = self.repo_dir(id).join("metadata.json");
        let meta_bytes = common_json::to_bytes(&repo.metadata)
            .map_err(|e| PvError::Internal(format!("serialize metadata: {e}")))?;
        fs::write(&meta_path, &meta_bytes)
            .map_err(|e| PvError::AtomicWriteFailed(format!("write metadata: {e}")))?;

        if let Ok(mut cache) = self.cache.write() {
            cache.insert(id.clone(), repo.clone());
        }

        Ok(repo.metadata)
    }

    /// Lists all repository ids.
    pub fn list(&self) -> Result<Vec<RepoId>, PvError> {
        let repos_dir = self.root.join("repos");
        if !repos_dir.exists() {
            return Ok(vec![]);
        }
        let mut ids = Vec::new();
        for entry in fs::read_dir(&repos_dir)
            .map_err(|e| PvError::Internal(format!("read repos dir: {e}")))?
        {
            let entry = entry.map_err(|e| PvError::Internal(format!("dir entry: {e}")))?;
            if let Ok(name) = entry.file_name().into_string()
                && let Ok(id) = name.parse::<RepoId>()
            {
                ids.push(id);
            }
        }
        ids.sort();
        Ok(ids)
    }

    /// Returns `true` if a repository with `id` exists.
    pub fn exists(&self, id: &RepoId) -> bool {
        if let Ok(cache) = self.cache.read()
            && cache.contains_key(id)
        {
            return true;
        }
        self.repo_dir(id).exists()
    }

    /// Returns the dedicated checkout root directory for a repository.
    pub fn checkout_root(&self, id: &RepoId) -> PathBuf {
        self.repo_dir(id).join("checkouts")
    }

    fn repo_dir(&self, id: &RepoId) -> PathBuf {
        self.root.join("repos").join(id.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    fn unique_test_dir(tag: &str) -> PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let pid = std::process::id();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        std::env::temp_dir().join(format!("pv_repos_{tag}_{pid}_{nanos}_{id}"))
    }

    #[test]
    fn create_and_get() {
        let dir = unique_test_dir("create");
        let store = RepoStore::open(&dir).unwrap();
        let id: RepoId = "my-repo".parse().unwrap();
        store
            .create(id.clone(), "My Repo".to_string(), None, 1000)
            .unwrap();
        let repo = store.get(&id).unwrap();
        assert_eq!(repo.metadata.name, "My Repo");
    }

    #[test]
    fn list_repos() {
        let dir = unique_test_dir("list");
        let store = RepoStore::open(&dir).unwrap();
        let id1: RepoId = "alpha".parse().unwrap();
        let id2: RepoId = "beta".parse().unwrap();
        store.create(id1, "Alpha".to_string(), None, 1).unwrap();
        store.create(id2, "Beta".to_string(), None, 2).unwrap();
        let list = store.list().unwrap();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn get_missing_returns_not_found() {
        let dir = unique_test_dir("missing");
        let store = RepoStore::open(&dir).unwrap();
        let id: RepoId = "ghost".parse().unwrap();
        let result = store.get(&id);
        assert!(matches!(result, Err(PvError::RepoNotFound(_))));
    }

    #[test]
    fn update_metadata() {
        let dir = unique_test_dir("update");
        let store = RepoStore::open(&dir).unwrap();
        let id: RepoId = "my-repo".parse().unwrap();
        store
            .create(id.clone(), "Old Name".to_string(), None, 1)
            .unwrap();
        let updated = store
            .update_metadata(&id, Some("New Name".to_string()), None, 2)
            .unwrap();
        assert_eq!(updated.name, "New Name");
    }
}
