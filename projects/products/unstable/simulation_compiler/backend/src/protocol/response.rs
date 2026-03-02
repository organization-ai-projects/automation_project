// projects/products/unstable/simulation_compiler/backend/src/protocol/response.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum CompilerResponse {
    Ok,
    Report { json: String },
    Error { message: String },
}
