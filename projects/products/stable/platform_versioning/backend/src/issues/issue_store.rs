// projects/products/stable/platform_versioning/backend/src/issues/issue_store.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::errors::PvError;
use crate::issues::{Issue, IssueId, IssueVisibility};
use crate::slices::SliceDefinition;

/// In-memory store for issues with thread-safe concurrent access.
///
/// All mutating operations emit audit events through the caller; this store
/// performs only persistence and visibility filtering.
pub struct IssueStore {
    inner: Arc<Mutex<HashMap<IssueId, Issue>>>,
}

impl IssueStore {
    /// Creates an empty issue store.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn lock(&self) -> Result<MutexGuard<'_, HashMap<IssueId, Issue>>, PvError> {
        self.inner
            .lock()
            .map_err(|_| PvError::Internal("issue store lock poisoned".to_string()))
    }

    /// Creates a new issue and stores it.
    ///
    /// Returns `Err` if an issue with the same `id` already exists.
    pub fn create(&self, issue: Issue) -> Result<Issue, PvError> {
        let mut guard = self.lock()?;
        if guard.contains_key(&issue.id) {
            return Err(PvError::Internal(format!(
                "issue '{}' already exists",
                issue.id
            )));
        }
        let id = issue.id.clone();
        guard.insert(id, issue.clone());
        Ok(issue)
    }

    /// Returns a clone of the issue with the given `id`.
    pub fn get(&self, id: &IssueId) -> Result<Issue, PvError> {
        let guard = self.lock()?;
        guard
            .get(id)
            .cloned()
            .ok_or_else(|| PvError::IssueNotFound(id.to_string()))
    }

    /// Lists all issues that `subject` may see according to `visibility`.
    pub fn list(&self, subject: &str, visibility: IssueVisibility) -> Result<Vec<Issue>, PvError> {
        let guard = self.lock()?;
        let issues = guard
            .values()
            .filter(|issue| match visibility {
                IssueVisibility::All => true,
                IssueVisibility::AssignedOrShared => issue.is_visible_to(subject),
            })
            .cloned()
            .collect();
        Ok(issues)
    }

    /// Adds `subject` to the assignees of the given issue.
    pub fn assign_user(&self, id: &IssueId, subject: String) -> Result<Issue, PvError> {
        let mut guard = self.lock()?;
        let issue = guard
            .get_mut(id)
            .ok_or_else(|| PvError::IssueNotFound(id.to_string()))?;
        if !issue.assignees.contains(&subject) {
            issue.assignees.push(subject);
        }
        Ok(issue.clone())
    }

    /// Adds `subject` to the shared-with list of the given issue.
    pub fn share_with(&self, id: &IssueId, subject: String) -> Result<Issue, PvError> {
        let mut guard = self.lock()?;
        let issue = guard
            .get_mut(id)
            .ok_or_else(|| PvError::IssueNotFound(id.to_string()))?;
        if !issue.shared_with.contains(&subject) {
            issue.shared_with.push(subject);
        }
        Ok(issue.clone())
    }

    /// Sets the [`SliceDefinition`] for the given issue.
    pub fn set_slice_definition(
        &self,
        id: &IssueId,
        definition: SliceDefinition,
    ) -> Result<Issue, PvError> {
        let mut guard = self.lock()?;
        let issue = guard
            .get_mut(id)
            .ok_or_else(|| PvError::IssueNotFound(id.to_string()))?;
        issue.slice_definition = Some(definition);
        Ok(issue.clone())
    }
}

impl Default for IssueStore {
    fn default() -> Self {
        Self::new()
    }
}
