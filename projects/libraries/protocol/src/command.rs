use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub payload: String,
    pub metadata: Metadata,
}

impl Command {
    pub fn validate(&self) -> bool {
        !self.name.trim().is_empty() && !self.payload.trim().is_empty()
    }
}
