// projects/products/core/engine/src/project_metadata.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        Self {
            id: String::from("default_id"),
            name: String::from("default_name"),
            description: String::from("default_description"),
            version: String::from("0.1.0"),
        }
    }
}
