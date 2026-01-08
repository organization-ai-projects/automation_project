// projects/products/varina/backend/src/classified_changes_ref.rs

//libraries
use git_lib::git_change::GitChange;

//internal
use crate::ClassifiedChanges;

/// Classification des changements selon policy (borrowed, zero-clone).
#[derive(Debug, Clone)]
pub struct ClassifiedChangesRef<'a> {
    pub relevant: Vec<&'a GitChange>,
    pub unrelated: Vec<&'a GitChange>,
    pub blocked: Vec<&'a GitChange>,
}

impl<'a> ClassifiedChangesRef<'a> {
    pub fn new() -> Self {
        Self {
            relevant: Vec::new(),
            unrelated: Vec::new(),
            blocked: Vec::new(),
        }
    }

    /// Convertit en version owning (clones explicites).
    pub fn to_owned(&self) -> ClassifiedChanges {
        ClassifiedChanges {
            relevant: self.relevant.iter().map(|&ch| ch.clone()).collect(),
            unrelated: self.unrelated.iter().map(|&ch| ch.clone()).collect(),
            blocked: self.blocked.iter().map(|&ch| ch.clone()).collect(),
        }
    }
}

impl<'a> Default for ClassifiedChangesRef<'a> {
    fn default() -> Self {
        Self::new()
    }
}
