use crate::json::Json; // Remplace serde_json::Value par Json centralisé
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplyResponse {
    pub result: String,
    pub payload: Option<Json>, // Utilisation du type Json centralisé
}
