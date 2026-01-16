use common_json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplyResponse {
    pub result: String,
    pub payload: Option<Json>, // Utilisation du type Json centralisé
}
