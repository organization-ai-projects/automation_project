use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Priority {
    pub name: String,
    pub weight: u8,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Aspirations {
    pub goal: Option<String>,
    pub priorities: Vec<Priority>,
}
