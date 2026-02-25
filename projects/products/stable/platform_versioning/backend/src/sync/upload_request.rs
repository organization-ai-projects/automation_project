// projects/products/stable/platform_versioning/backend/src/sync/upload_request.rs
use serde::{Deserialize, Serialize};

use crate::objects::Object;
use crate::sync::RefUpdate;

/// A request to push objects and optionally update refs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadRequest {
    /// Objects to store in the server's object store.
    pub objects: Vec<Object>,
    /// Ref updates to apply after all objects are stored.
    pub ref_updates: Vec<RefUpdate>,
}
