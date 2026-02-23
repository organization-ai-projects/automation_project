// projects/products/unstable/platform_versioning/backend/src/objects/object_store.rs
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use crate::errors::PvError;
use crate::ids::ObjectId;
use crate::objects::Object;

/// Content-addressed, append-only object store.
///
/// # Atomicity
/// Writes use a temp-file-then-rename pattern so readers never observe partial
/// objects. If the rename fails, the temporary file is cleaned up.
///
/// # On-disk layout
/// ```text
/// <root>/
///   objects/
///     <2-char prefix>/
///       <remaining 62 hex chars>     (bincode-encoded Object)
/// ```
#[derive(Clone)]
pub struct ObjectStore {
    root: PathBuf,
    cache: Arc<RwLock<HashMap<ObjectId, Object>>>,
}

impl ObjectStore {
    /// Opens (or creates) an object store rooted at `root`.
    pub fn open(root: impl AsRef<Path>) -> Result<Self, PvError> {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(root.join("objects"))
            .map_err(|e| PvError::AtomicWriteFailed(format!("create objects dir: {e}")))?;
        Ok(Self {
            root,
            cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Stores `object` and returns its id.
    ///
    /// If an object with the same id already exists, the write is skipped
    /// (objects are immutable and content-addressed).
    pub fn write(&self, object: Object) -> Result<ObjectId, PvError> {
        let id = match &object {
            Object::Blob(b) => b.id.as_object_id().clone(),
            Object::Tree(t) => t.id.as_object_id().clone(),
            Object::Commit(c) => c.id.as_object_id().clone(),
        };

        // Skip if already present.
        let path = self.object_path(&id);
        if path.exists() {
            self.cache_insert(id.clone(), object);
            return Ok(id);
        }

        let encoded = serde_json::to_vec(&object)
            .map_err(|e| PvError::Internal(format!("encode object: {e}")))?;

        self.atomic_write(&path, &encoded)?;
        self.cache_insert(id.clone(), object);
        Ok(id)
    }

    /// Reads and validates the object with the given `id`.
    pub fn read(&self, id: &ObjectId) -> Result<Object, PvError> {
        // Check cache first.
        if let Ok(cache) = self.cache.read() {
            if let Some(obj) = cache.get(id) {
                return Ok(obj.clone());
            }
        }

        let path = self.object_path(id);
        let bytes = fs::read(&path).map_err(|_| PvError::ObjectNotFound(id.to_string()))?;

        let (object, _): (Object, _) = serde_json::from_slice(&bytes)
            .map(|o| (o, ()))
            .map_err(|e| PvError::CorruptObject(format!("decode {id}: {e}")))?;

        if !object.verify() {
            return Err(PvError::CorruptObject(format!(
                "integrity check failed for {id}"
            )));
        }

        self.cache_insert(id.clone(), object.clone());
        Ok(object)
    }

    /// Returns `true` if an object with `id` exists in the store.
    pub fn exists(&self, id: &ObjectId) -> bool {
        if let Ok(cache) = self.cache.read() {
            if cache.contains_key(id) {
                return true;
            }
        }
        self.object_path(id).exists()
    }

    fn object_path(&self, id: &ObjectId) -> PathBuf {
        let hex = id.as_str();
        let (prefix, rest) = hex.split_at(2);
        self.root.join("objects").join(prefix).join(rest)
    }

    fn atomic_write(&self, path: &Path, data: &[u8]) -> Result<(), PvError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| PvError::AtomicWriteFailed(format!("create dir: {e}")))?;
        }

        let pid = std::process::id();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        let tmp_name = format!(
            ".{}.tmp-{}-{}",
            path.file_name().and_then(|n| n.to_str()).unwrap_or("obj"),
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
            file.write_all(data)
                .map_err(|e| PvError::AtomicWriteFailed(format!("write tmp: {e}")))?;
            file.sync_all()
                .map_err(|e| PvError::AtomicWriteFailed(format!("sync tmp: {e}")))?;
            fs::rename(&tmp_path, path)
                .map_err(|e| PvError::AtomicWriteFailed(format!("rename tmp: {e}")))?;
            Ok(())
        })();

        if result.is_err() {
            let _ = fs::remove_file(&tmp_path);
        }
        result
    }

    fn cache_insert(&self, id: ObjectId, object: Object) {
        if let Ok(mut cache) = self.cache.write() {
            cache.insert(id, object);
        }
    }
}

/// Required by bincode for Object serialization.
impl Serialize for ObjectStore {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.root.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ObjectStore {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let root = PathBuf::deserialize(deserializer)?;
        Self::open(root).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::Blob;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    fn unique_test_dir() -> PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let pid = std::process::id();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        std::env::temp_dir().join(format!("pv_obj_store_{pid}_{nanos}_{id}"))
    }

    fn test_store() -> (PathBuf, ObjectStore) {
        let dir = unique_test_dir();
        let store = ObjectStore::open(&dir).unwrap();
        (dir, store)
    }

    #[test]
    fn write_and_read_blob() {
        let (_dir, store) = test_store();
        let blob = Blob::from_bytes(b"hello".to_vec());
        let id = blob.id.as_object_id().clone();
        store.write(Object::Blob(blob)).unwrap();
        let obj = store.read(&id).unwrap();
        assert_eq!(obj.kind(), crate::objects::ObjectKind::Blob);
    }

    #[test]
    fn exists_returns_true_after_write() {
        let (_dir, store) = test_store();
        let blob = Blob::from_bytes(b"data".to_vec());
        let id = blob.id.as_object_id().clone();
        store.write(Object::Blob(blob)).unwrap();
        assert!(store.exists(&id));
    }

    #[test]
    fn read_missing_returns_not_found() {
        let (_dir, store) = test_store();
        let id = ObjectId::from_bytes(&[0xddu8; 32]);
        let result = store.read(&id);
        assert!(matches!(result, Err(PvError::ObjectNotFound(_))));
    }

    #[test]
    fn corrupt_bytes_fail_integrity() {
        let (_dir, store) = test_store();
        let blob = Blob::from_bytes(b"important data".to_vec());
        let id = blob.id.as_object_id().clone();
        store.write(Object::Blob(blob)).unwrap();

        // Corrupt the file on disk.
        let path = {
            let hex = id.as_str();
            let (prefix, rest) = hex.split_at(2);
            store.root.join("objects").join(prefix).join(rest)
        };
        let mut bytes = fs::read(&path).unwrap();
        bytes[0] ^= 0xff;
        fs::write(&path, &bytes).unwrap();

        // Flush the cache to force a disk read.
        store.cache.write().unwrap().clear();

        let result = store.read(&id);
        assert!(
            matches!(result, Err(PvError::CorruptObject(_)))
                || matches!(result, Err(PvError::ObjectNotFound(_)))
        );
    }
}
