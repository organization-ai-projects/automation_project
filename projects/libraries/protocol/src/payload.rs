//projects/libraries/protocol/src/payload.rs
use crate::json::Json; // Remplace serde_json::Value par Json centralisé
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    pub payload_type: Option<String>,
    pub payload: Option<Json>,
}
