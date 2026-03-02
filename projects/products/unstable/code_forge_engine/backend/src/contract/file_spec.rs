// projects/products/unstable/code_forge_engine/backend/src/contract/file_spec.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSpec {
    pub path: String,
    pub primary_type: String,
    pub content_template: String,
}
