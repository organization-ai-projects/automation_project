//! projects/products/varina/backend/src/classified_changes_ref.rs

//internal
use crate::ClassifiedChanges;

/// Classification of changes according to policy (borrowed, zero-clone).
#[derive(Debug, Clone)]
pub struct ClassifiedChangesRef {
    pub relevant: Vec<String>,
    pub unrelated: Vec<String>,
    pub blocked: Vec<String>,
}

impl ClassifiedChangesRef {
    pub fn new() -> Self {
        Self {
            relevant: Vec::new(),
            unrelated: Vec::new(),
            blocked: Vec::new(),
        }
    }

    /// Converts to an owning version (explicit clones).
    pub fn to_owned(&self) -> ClassifiedChanges {
        ClassifiedChanges {
            relevant: self.relevant.to_vec(),
            unrelated: self.unrelated.to_vec(),
            blocked: self.blocked.to_vec(),
        }
    }
}

impl Default for ClassifiedChangesRef {
    fn default() -> Self {
        Self::new()
    }
}
