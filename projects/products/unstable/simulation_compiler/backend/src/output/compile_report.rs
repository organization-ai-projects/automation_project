// projects/products/unstable/simulation_compiler/backend/src/output/compile_report.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileReport {
    pub success: bool,
    pub artifact_count: usize,
    pub manifest_hash: String,
    pub diagnostics: Vec<String>,
}
