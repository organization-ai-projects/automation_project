use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatyReport {
    pub treaty_id: String,
    pub kind: String,
    pub parties: Vec<String>,
    pub start_turn: u64,
    pub end_turn: Option<u64>,
}
