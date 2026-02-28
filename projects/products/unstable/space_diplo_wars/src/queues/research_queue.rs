use serde::{Deserialize, Serialize};

use super::queue_id::QueueId;

/// Research queue for tech advancement.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ResearchQueue {
    pub id: Option<QueueId>,
    pub items: Vec<String>,
}

impl ResearchQueue {
    pub fn new() -> Self {
        Self::default()
    }
}
