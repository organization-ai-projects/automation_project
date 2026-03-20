use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::expert_id::ExpertId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertOutput {
    pub expert_id: ExpertId,
    pub content: String,
    pub confidence: f64,
    pub metadata: HashMap<String, String>,
    pub trace: Vec<String>,
}
