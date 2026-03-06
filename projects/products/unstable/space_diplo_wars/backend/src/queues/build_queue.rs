use serde::{Deserialize, Serialize};

use super::queue_id::QueueId;

/// Build queue for ship or structure construction.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuildQueue {
    pub id: Option<QueueId>,
    pub items: Vec<String>,
}

impl BuildQueue {
    pub fn new() -> Self {
        Self::default()
    }
}
