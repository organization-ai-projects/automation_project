use serde::{Deserialize, Serialize};
use serde_json::Value; // Ajout pour payload générique

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplyResponse {
    pub result: String,
    pub payload: Option<Value>, // Remplacement par un payload générique
}
