//! projects/products/varina/backend/src/classified_changes.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiedChanges {
    pub relevant: Vec<String>,
    pub unrelated: Vec<String>,
    pub blocked: Vec<String>,
}
