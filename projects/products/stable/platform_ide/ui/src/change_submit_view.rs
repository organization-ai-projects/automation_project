// projects/products/stable/platform_ide/ui/src/change_submit_view.rs
use serde::{Deserialize, Serialize};

/// The change submission view state.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ChangeSubmitView {
    /// The commit message for the submission.
    pub message: String,
    /// The list of file paths staged for submission (always allowed paths).
    pub staged_paths: Vec<String>,
    /// Whether a submission is in progress.
    pub submitting: bool,
    /// The commit ID returned after a successful submission.
    pub last_commit_id: Option<String>,
}

impl ChangeSubmitView {
    /// Stages paths for submission.
    pub fn stage(&mut self, paths: Vec<String>, message: String) {
        self.staged_paths = paths;
        self.message = message;
    }

    /// Records a successful submission.
    pub fn on_submitted(&mut self, commit_id: String) {
        self.submitting = false;
        self.last_commit_id = Some(commit_id);
        self.staged_paths.clear();
    }
}
