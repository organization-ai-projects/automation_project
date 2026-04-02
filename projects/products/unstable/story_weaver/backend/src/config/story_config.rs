use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryConfig {
    pub seed: u64,
    pub max_steps: u64,
}
