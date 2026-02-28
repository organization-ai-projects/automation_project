// projects/products/unstable/protocol_builder/backend/src/protocol/request.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IpcRequest {
    LoadSchema { path: String },
    ValidateSchema,
    GenerateDryRun,
    GenerateWrite { out_dir: String },
    GetReport,
    Shutdown,
}
