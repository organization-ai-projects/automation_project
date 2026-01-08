// projects/libraries/protocol/src/preview_response.rs
use serde::{Deserialize, Serialize};
use serde_json::Value; // Ajout pour payload générique

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreviewResponse {
    pub summary: String,
    pub payload: Option<Value>, // Remplacement par un payload générique
}
