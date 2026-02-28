// projects/products/unstable/simulation_compiler/backend/src/protocol/request.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum CompilerRequest {
    LoadDsl { source: String },
    Validate,
    CompileDryRun,
    CompileWrite { out_dir: String },
    GetReport,
}
