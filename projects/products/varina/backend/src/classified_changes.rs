use git_lib::git_change::GitChange;
// projects/products/varina/backend/src/classified_changes.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiedChanges {
    pub relevant: Vec<GitChange>,
    pub unrelated: Vec<GitChange>,
    pub blocked: Vec<GitChange>,
}
