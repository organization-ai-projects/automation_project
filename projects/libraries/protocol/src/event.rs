use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub data: String,
}

impl Event {
    pub fn validate(&self) -> bool {
        !self.name.trim().is_empty() && !self.data.trim().is_empty()
    }
}
