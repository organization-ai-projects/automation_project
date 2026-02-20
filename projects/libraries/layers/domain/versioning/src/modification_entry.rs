// projects/libraries/layers/domain/versioning/src/modification_entry.rs

use crate::modification_category::ModificationCategory;
use serde::{Deserialize, Serialize};

/// Represents a single modification entry in the revision log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModificationEntry {
    description: String,
    category: ModificationCategory,
}

impl ModificationEntry {
    pub fn create(description: String, category: ModificationCategory) -> Self {
        Self {
            description,
            category,
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_category(&self) -> &ModificationCategory {
        &self.category
    }
}
