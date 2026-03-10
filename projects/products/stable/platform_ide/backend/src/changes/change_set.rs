//! projects/products/stable/platform_ide/backend/src/changes/change_set.rs
use crate::changes::patch_entry::PatchEntry;
use crate::editor::FileBuffer;
use crate::errors::IdeError;

/// A set of file patches scoped to a single issue.
///
/// A `ChangeSet` can only contain entries for paths that have passed through
/// the slice manifest enforcement (`AllowedPath`). Attempting to collect from
/// a buffer whose path has not been validated is a compile-time impossibility
/// because `FileBuffer` only stores `AllowedPath`.
#[derive(Debug, Default)]
pub struct ChangeSet {
    entries: Vec<PatchEntry>,
}

impl ChangeSet {
    /// Creates a new, empty change set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a dirty buffer to the change set.
    ///
    /// Returns `false` (skips) if the buffer is not dirty to avoid no-op
    /// round-trips to the platform.
    pub fn add_buffer(&mut self, buf: &FileBuffer) -> bool {
        if !buf.is_dirty() {
            return false;
        }
        self.entries.push(PatchEntry {
            path: buf.path.clone(),
            content: buf.content().to_vec(),
        });
        true
    }

    /// Returns `true` if the change set contains no patches.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the number of patches in the change set.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns the patch entries.
    pub fn entries(&self) -> &[PatchEntry] {
        &self.entries
    }

    /// Validates that the change set is non-empty before submission.
    pub fn validate(&self) -> Result<(), IdeError> {
        if self.is_empty() {
            return Err(IdeError::EmptyChangeSet);
        }
        Ok(())
    }
}
