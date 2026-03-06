// projects/products/unstable/protocol_builder/ui/src/transport/request.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Request {
    LoadSchema { path: String },
    ValidateSchema,
    GenerateDryRun,
    GenerateWrite { out_dir: String },
    GetReport,
    Shutdown,
}
