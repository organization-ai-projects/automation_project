use serde::{Deserialize, Serialize};

use crate::dsl::Script;
use crate::events::StoryEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayFile {
    pub seed: u64,
    pub script: Script,
    pub events: Vec<StoryEvent>,
}

impl ReplayFile {
    pub fn new(seed: u64, script: Script, events: Vec<StoryEvent>) -> Self {
        Self {
            seed,
            script,
            events,
        }
    }
}
