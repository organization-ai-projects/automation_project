// projects/libraries/protocol/src/preview_response.rs
use crate::json::Json; // Remplace serde_json::Value par Json centralisé
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreviewResponse {
    pub summary: String,
    pub payload: Option<Json>, // Utilisation du type Json centralisé
}
