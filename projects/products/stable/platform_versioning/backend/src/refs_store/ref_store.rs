// projects/products/stable/platform_versioning/backend/src/refs_store/ref_store.rs
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::errors::PvError;
use crate::ids::CommitId;
use crate::objects::ObjectStore;
use crate::refs_store::{HeadState, RefName, RefTarget};

/// Mutable ref store with atomic update semantics.
///
/// # On-disk layout
/// ```text
/// <root>/
///   HEAD               (serialized HeadState as JSON)
///   refs/
///     heads/<name>     (serialized RefTarget as JSON)
///     tags/<name>      (serialized RefTarget as JSON)
/// ```
///
/// # Atomicity
/// All updates use a temp-file-then-rename pattern so concurrent readers never
/// observe a partially-written ref file.
#[derive(Clone)]
pub struct RefStore {
    root: PathBuf,
}

impl Serialize for RefStore {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.root.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RefStore {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let root = PathBuf::deserialize(deserializer)?;
        Self::open(root).map_err(serde::de::Error::custom)
    }
}

impl RefStore {
    /// Opens (or creates) a ref store rooted at `root`.
    pub fn open(root: impl AsRef<Path>) -> Result<Self, PvError> {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(root.join("refs").join("heads"))
            .map_err(|e| PvError::AtomicWriteFailed(format!("create refs/heads: {e}")))?;
        fs::create_dir_all(root.join("refs").join("tags"))
            .map_err(|e| PvError::AtomicWriteFailed(format!("create refs/tags: {e}")))?;
        Ok(Self { root })
    }

    /// Returns the current HEAD state.
    pub fn read_head(&self) -> Result<HeadState, PvError> {
        let path = self.root.join("HEAD");
        if !path.exists() {
            // Default: unborn branch named "heads/main"
            let default_branch: RefName = "heads/main"
                .parse()
                .map_err(|e| PvError::Internal(format!("default branch parse: {e}")))?;
            return Ok(HeadState::Unborn(default_branch));
        }
        self.read_json(&path)
    }

    /// Atomically updates HEAD.
    pub fn write_head(&self, state: &HeadState) -> Result<(), PvError> {
        let path = self.root.join("HEAD");
        self.write_json(&path, state)
    }

    /// Reads the target for the given ref.
    pub fn read_ref(&self, name: &RefName) -> Result<RefTarget, PvError> {
        let path = self.ref_path(name);
        if !path.exists() {
            return Err(PvError::RefNotFound(name.to_string()));
        }
        self.read_json(&path)
    }

    /// Atomically writes or updates a ref.
    ///
    /// # Fast-forward enforcement
    /// If `force` is `false`, the new target must descend from the current target
    /// (i.e. the update must be a fast-forward). Pass `object_store` for
    /// reachability checking; if `None`, ancestry is not validated.
    pub fn write_ref(
        &self,
        name: &RefName,
        target: &RefTarget,
        force: bool,
        object_store: Option<&ObjectStore>,
    ) -> Result<(), PvError> {
        let path = self.ref_path(name);

        if !force && let Ok(current) = self.read_ref(name) {
            let current_id = current.commit_id();
            let new_id = target.commit_id();
            if let Some(store) = object_store
                && !is_ancestor(current_id, new_id, store)
            {
                return Err(PvError::NonFastForward(format!(
                    "update of '{}' is not a fast-forward",
                    name
                )));
            }
        }

        // Ensure target commit exists.
        if let Some(store) = object_store {
            let id = target.commit_id().as_object_id();
            if !store.exists(id) {
                return Err(PvError::CommitNotFound(target.commit_id().to_string()));
            }
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| PvError::AtomicWriteFailed(format!("mkdir: {e}")))?;
        }
        self.write_json(&path, target)
    }

    /// Returns all refs as a map from name to target.
    pub fn list_refs(&self) -> Result<HashMap<RefName, RefTarget>, PvError> {
        let mut out = HashMap::new();
        for prefix in &["heads", "tags"] {
            let dir = self.root.join("refs").join(prefix);
            if !dir.exists() {
                continue;
            }
            for entry in fs::read_dir(&dir)
                .map_err(|e| PvError::Internal(format!("read refs/{prefix}: {e}")))?
            {
                let entry = entry.map_err(|e| PvError::Internal(format!("dir entry: {e}")))?;
                let file_name = entry.file_name();
                let short = file_name.to_string_lossy();
                let full = format!("{prefix}/{short}");
                if let Ok(name) = full.parse::<RefName>()
                    && let Ok(target) = self.read_ref(&name)
                {
                    out.insert(name, target);
                }
            }
        }
        Ok(out)
    }

    fn ref_path(&self, name: &RefName) -> PathBuf {
        self.root.join("refs").join(name.as_str())
    }

    fn read_json<T: for<'de> Deserialize<'de>>(&self, path: &Path) -> Result<T, PvError> {
        let bytes = fs::read(path)
            .map_err(|e| PvError::Internal(format!("read {}: {e}", path.display())))?;
        common_json::from_slice(&bytes)
            .map_err(|e| PvError::Internal(format!("parse {}: {e}", path.display())))
    }

    fn write_json<T: Serialize>(&self, path: &Path, value: &T) -> Result<(), PvError> {
        let bytes = common_json::to_bytes(value)
            .map_err(|e| PvError::Internal(format!("serialize: {e}")))?;

        let pid = std::process::id();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        let tmp_name = format!(
            ".{}.tmp-{}-{}",
            path.file_name().and_then(|n| n.to_str()).unwrap_or("ref"),
            pid,
            nanos
        );
        let tmp_path = path
            .parent()
            .map(|p| p.join(&tmp_name))
            .unwrap_or_else(|| PathBuf::from(&tmp_name));

        let result = (|| -> Result<(), PvError> {
            let mut file = fs::File::create(&tmp_path)
                .map_err(|e| PvError::AtomicWriteFailed(format!("create tmp: {e}")))?;
            file.write_all(&bytes)
                .map_err(|e| PvError::AtomicWriteFailed(format!("write tmp: {e}")))?;
            file.sync_all()
                .map_err(|e| PvError::AtomicWriteFailed(format!("sync tmp: {e}")))?;
            fs::rename(&tmp_path, path)
                .map_err(|e| PvError::AtomicWriteFailed(format!("rename: {e}")))?;
            Ok(())
        })();

        if result.is_err() {
            let _ = fs::remove_file(&tmp_path);
        }
        result
    }
}

/// Returns `true` if `ancestor` is reachable from `descendant` by following parent links.
fn is_ancestor(ancestor: &CommitId, descendant: &CommitId, store: &ObjectStore) -> bool {
    if ancestor == descendant {
        return true;
    }
    use crate::objects::Object;
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    queue.push_back(descendant.clone());
    let mut visited = std::collections::HashSet::new();

    while let Some(current) = queue.pop_front() {
        if current == *ancestor {
            return true;
        }
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());
        let id = current.as_object_id().clone();
        if let Ok(Object::Commit(c)) = store.read(&id) {
            for parent in c.parent_ids {
                queue.push_back(parent);
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    fn unique_test_dir() -> PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let pid = std::process::id();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        std::env::temp_dir().join(format!("pv_ref_store_{pid}_{nanos}_{id}"))
    }

    fn make_commit_id(byte: u8) -> CommitId {
        CommitId::from(crate::ids::ObjectId::from_bytes(&[byte; 32]))
    }

    #[test]
    fn default_head_is_unborn() {
        let dir = unique_test_dir();
        let store = RefStore::open(&dir).unwrap();
        let head = store.read_head().unwrap();
        assert!(matches!(head, HeadState::Unborn(_)));
    }

    #[test]
    fn write_and_read_head() {
        let dir = unique_test_dir();
        let store = RefStore::open(&dir).unwrap();
        let name: RefName = "heads/main".parse().unwrap();
        store.write_head(&HeadState::Branch(name.clone())).unwrap();
        let head = store.read_head().unwrap();
        assert_eq!(head, HeadState::Branch(name));
    }

    #[test]
    fn read_missing_ref() {
        let dir = unique_test_dir();
        let store = RefStore::open(&dir).unwrap();
        let name: RefName = "heads/missing".parse().unwrap();
        let result = store.read_ref(&name);
        assert!(matches!(result, Err(PvError::RefNotFound(_))));
    }

    #[test]
    fn write_and_read_ref() {
        let dir = unique_test_dir();
        let store = RefStore::open(&dir).unwrap();
        let name: RefName = "heads/main".parse().unwrap();
        let commit_id = make_commit_id(0xaa);
        let target = RefTarget::Commit(commit_id.clone());
        store.write_ref(&name, &target, true, None).unwrap();
        let read = store.read_ref(&name).unwrap();
        assert_eq!(read.commit_id(), &commit_id);
    }

    #[test]
    fn non_fast_forward_rejected() {
        let dir = unique_test_dir();
        let store = RefStore::open(&dir).unwrap();
        let name: RefName = "heads/main".parse().unwrap();
        let id1 = make_commit_id(0x01);
        let id2 = make_commit_id(0x02);
        // Write initial ref.
        store
            .write_ref(&name, &RefTarget::Commit(id1), true, None)
            .unwrap();
        // Try non-fast-forward without force (no object store for ancestry check, so
        // is_ancestor returns false because id1 != id2 and no parents can be followed).
        let obj_root = unique_test_dir();
        let obj_store = ObjectStore::open(&obj_root).unwrap();
        let result = store.write_ref(&name, &RefTarget::Commit(id2), false, Some(&obj_store));
        assert!(matches!(result, Err(PvError::NonFastForward(_))));
    }
}
