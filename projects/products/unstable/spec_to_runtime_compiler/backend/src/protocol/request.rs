use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum CompilerRequest {
    LoadSpec { source: String },
    ValidateSpec,
    CompileDryRun,
    CompileWrite { out_dir: String },
    GetCompileReport,
    Shutdown,
}
