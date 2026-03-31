use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvariantSpec {
    pub name: String,
    pub description: String,
}
