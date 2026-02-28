// projects/products/unstable/code_forge_engine/backend/src/protocol/request.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Request {
    LoadContract { path: String },
    ValidateContract,
    PreviewLayout,
    Generate { out_dir: String, mode: String },
    GetManifest,
    Shutdown,
}
