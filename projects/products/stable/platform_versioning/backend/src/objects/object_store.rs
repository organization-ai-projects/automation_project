// projects/products/stable/platform_versioning/backend/src/objects/object_store.rs
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use crate::errors::PvError;
use crate::ids::ObjectId;
use crate::nonce::next_nonce;
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

        let encoded = common_json::to_bytes(&object)
            .map_err(|e| PvError::Internal(format!("encode object: {e}")))?;

        self.atomic_write(&path, &encoded)?;
        self.cache_insert(id.clone(), object);
        Ok(id)
    }

    /// Reads and validates the object with the given `id`.
    pub fn read(&self, id: &ObjectId) -> Result<Object, PvError> {
        // Check cache first.
        if let Ok(cache) = self.cache.read()
            && let Some(obj) = cache.get(id)
        {
            return Ok(obj.clone());
        }

        let path = self.object_path(id);
        let bytes = fs::read(&path).map_err(|_| PvError::ObjectNotFound(id.to_string()))?;

        let object: Object = common_json::from_slice(&bytes)
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
        if let Ok(cache) = self.cache.read()
            && cache.contains_key(id)
        {
            return true;
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
        let nanos = next_nonce();
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
            drop(fs::remove_file(&tmp_path));
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
