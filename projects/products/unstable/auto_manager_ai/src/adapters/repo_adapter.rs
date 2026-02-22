// projects/products/unstable/auto_manager_ai/src/adapters/repo_adapter.rs

use super::repo_context::RepoContext;
use std::fs;
use std::path::{Path, PathBuf};

/// Repository adapter (read-only in V0)
#[derive(Debug)]
pub struct RepoAdapter {
    repo_path: PathBuf,
}

impl RepoAdapter {
    /// Create a new repository adapter
    pub fn new(repo_path: PathBuf) -> Self {
        Self { repo_path }
    }

    /// Get repository context (read-only)
    pub fn get_context(&self) -> Result<RepoContext, String> {
        if !self.repo_path.exists() {
            return Err(format!(
                "Repository path does not exist: {:?}",
                self.repo_path
            ));
        }

        let tracked_files = self.list_tracked_files()?;

        Ok(RepoContext {
            root: self.repo_path.clone(),
            tracked_files,
            mediated_by_engine: false,
        })
    }

    /// List tracked files (read-only operation)
    fn list_tracked_files(&self) -> Result<Vec<String>, String> {
        // Simple file listing for V0 (not using git)
        let mut files = Vec::new();
        self.walk_dir(&self.repo_path, &mut files)?;
        Ok(files)
    }

    /// Recursively walk directory (helper)
    fn walk_dir(&self, dir: &Path, files: &mut Vec<String>) -> Result<(), String> {
        if !dir.is_dir() {
            return Ok(());
        }

        let entries =
            fs::read_dir(dir).map_err(|e| format!("Failed to read directory {:?}: {}", dir, e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();

            // Skip hidden files and directories
            if let Some(name) = path.file_name()
                && name.to_string_lossy().starts_with('.')
            {
                continue;
            }

            if path.is_file() {
                if let Ok(rel_path) = path.strip_prefix(&self.repo_path) {
                    files.push(rel_path.to_string_lossy().to_string());
                }
            } else if path.is_dir() {
                self.walk_dir(&path, files)?;
            }
        }

        Ok(())
    }

    /// Read a file from the repository (read-only)
    #[allow(dead_code)] // Reserved for future use in planners and diagnostics
    pub fn read_file(&self, path: &str) -> Result<String, String> {
        let full_path = self.repo_path.join(path);
        fs::read_to_string(&full_path)
            .map_err(|e| format!("Failed to read file {:?}: {}", full_path, e))
    }
}
