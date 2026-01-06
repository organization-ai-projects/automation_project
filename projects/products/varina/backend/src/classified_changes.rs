use crate::git_github::GitChange;

/// Classification des changements selon policy.
#[derive(Debug, Clone)]
pub struct ClassifiedChanges {
    pub relevant: Vec<GitChange>,
    pub unrelated: Vec<GitChange>,
    pub blocked: Vec<GitChange>,
}