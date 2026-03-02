use crate::diagnostics::error::AgentError;
use crate::patch::file_edit::FileEdit;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditSummary {
    pub path: String,
    pub bytes_written: usize,
}

pub struct PatchApplier;

impl PatchApplier {
    pub fn new() -> Self {
        Self
    }

    /// Applies a slice of `FileEdit`s under `root`, writing each edit to disk.
    /// Returns a summary of every applied edit.
    pub fn apply(&self, root: &Path, edits: &[FileEdit]) -> Result<Vec<EditSummary>, AgentError> {
        let mut summaries = Vec::new();
        for edit in edits {
            let full_path = root.join(&edit.path);
            std::fs::write(&full_path, &edit.new_content)
                .map_err(|e| AgentError::Patch(format!("failed to write {}: {e}", edit.path)))?;
            summaries.push(EditSummary {
                path: edit.path.clone(),
                bytes_written: edit.new_content.len(),
            });
        }
        Ok(summaries)
    }
}

impl Default for PatchApplier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_dir(suffix: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let pid = std::process::id();
        let dir = std::env::temp_dir().join(format!("dev_agent_patch_{suffix}_{pid}_{nanos}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn apply_writes_content() {
        let dir = temp_dir("write");
        let edits = vec![FileEdit::new("test.txt", "hello world")];
        let summaries = PatchApplier::new().apply(&dir, &edits).unwrap();
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].path, "test.txt");
        assert_eq!(summaries[0].bytes_written, 11);
        assert_eq!(
            fs::read_to_string(dir.join("test.txt")).unwrap(),
            "hello world"
        );
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn apply_multiple_edits() {
        let dir = temp_dir("multi");
        let edits = vec![FileEdit::new("a.txt", "aaa"), FileEdit::new("b.txt", "bb")];
        let summaries = PatchApplier::new().apply(&dir, &edits).unwrap();
        assert_eq!(summaries.len(), 2);
        assert_eq!(fs::read_to_string(dir.join("a.txt")).unwrap(), "aaa");
        assert_eq!(fs::read_to_string(dir.join("b.txt")).unwrap(), "bb");
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn apply_missing_dir_returns_error() {
        let dir = std::path::PathBuf::from("/nonexistent_path_xyz_abc");
        let edits = vec![FileEdit::new("test.txt", "x")];
        assert!(PatchApplier::new().apply(&dir, &edits).is_err());
    }
}
