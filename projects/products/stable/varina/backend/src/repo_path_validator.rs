// projects/products/stable/varina/backend/src/repo_path_validator.rs
use std::path::{Path, PathBuf};

use crate::validation_error::{
    ValidationError, E_REPO_PATH_INVALID_FORMAT, E_REPO_PATH_NOT_WHITELISTED,
    E_REPO_PATH_TRAVERSAL,
};

/// Result type for repo path validation
pub type ValidationResult<T> = Result<T, ValidationError>;

/// Repository path validator with whitelist support
pub struct RepoPathValidator {
    whitelist: Vec<PathBuf>,
}

impl RepoPathValidator {
    /// Create a new validator with the given whitelist of allowed paths
    #[allow(dead_code)]
    pub fn new(whitelist: Vec<PathBuf>) -> Self {
        // Canonicalize whitelist paths to handle symlinks correctly
        let whitelist = whitelist
            .into_iter()
            .filter_map(|path| {
                if path.exists() {
                    path.canonicalize().ok()
                } else {
                    // Keep non-existent paths as-is for flexibility
                    Some(path)
                }
            })
            .collect();
        Self { whitelist }
    }

    /// Create a validator with default whitelist
    pub fn with_default_whitelist() -> Self {
        // Default whitelist allows common development directories
        let whitelist = vec![
            PathBuf::from("/home"),
            PathBuf::from("/tmp"),
            PathBuf::from("/workspace"),
        ];
        Self::new(whitelist)
    }

    /// Validate and normalize a repository path
    pub fn validate(&self, repo_path: &str) -> ValidationResult<PathBuf> {
        // Check for empty or whitespace-only paths
        if repo_path.trim().is_empty() {
            return Err(ValidationError::new(
                E_REPO_PATH_INVALID_FORMAT,
                "Repository path cannot be empty".to_string(),
            ));
        }

        // Normalize the path first
        let path = Path::new(repo_path);
        let normalized = if path.is_absolute() {
            path.to_path_buf()
        } else {
            // Relative paths are converted to absolute based on current directory
            std::env::current_dir()
                .map_err(|e| {
                    ValidationError::new(
                        E_REPO_PATH_INVALID_FORMAT,
                        format!("Failed to get current directory: {}", e),
                    )
                })?
                .join(path)
        };

        // Canonicalize to resolve symlinks and detect path traversal
        // This is more robust than string-based checks for '..'
        let canonical = if normalized.exists() {
            normalized.canonicalize().map_err(|e| {
                ValidationError::new(
                    E_REPO_PATH_INVALID_FORMAT,
                    format!("Failed to canonicalize path: {}", e),
                )
            })?
        } else {
            // For non-existent paths, we still need to detect path traversal
            // Normalize components to detect traversal without filesystem access
            self.normalize_non_existent_path(&normalized)?
        };

        // Check against whitelist
        if !self.is_whitelisted(&canonical) {
            return Err(ValidationError::new(
                E_REPO_PATH_NOT_WHITELISTED,
                format!(
                    "Repository path '{}' is not in the whitelist",
                    canonical.display()
                ),
            ));
        }

        Ok(canonical)
    }

    /// Normalize a non-existent path to detect traversal attempts
    fn normalize_non_existent_path(&self, path: &Path) -> ValidationResult<PathBuf> {
        let mut normalized = PathBuf::new();
        for component in path.components() {
            match component {
                std::path::Component::ParentDir => {
                    // Pop the last component instead of rejecting
                    // This allows paths like /home/user/../user/repo to work
                    if !normalized.pop() {
                        // Can't go beyond root, this is a traversal attempt
                        return Err(ValidationError::new(
                            E_REPO_PATH_TRAVERSAL,
                            "Path traversal detected: attempt to navigate above root directory".to_string(),
                        ));
                    }
                }
                _ => normalized.push(component),
            }
        }
        Ok(normalized)
    }

    /// Check if a path is within any whitelisted directory
    fn is_whitelisted(&self, path: &Path) -> bool {
        if self.whitelist.is_empty() {
            // If whitelist is empty, allow all paths
            return true;
        }

        // Only allow paths that are children of whitelisted directories
        self.whitelist.iter().any(|allowed| path.starts_with(allowed))
    }
}

impl Default for RepoPathValidator {
    fn default() -> Self {
        Self::with_default_whitelist()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_traversal_rejected() {
        let validator = RepoPathValidator::default();
        // This path tries to escape to /opt which is not in the default whitelist
        let result = validator.validate("/home/user/../../opt/config");
        assert!(result.is_err());
        let err = result.unwrap_err();
        // After normalization, this should be caught by whitelist check
        assert_eq!(err.code, E_REPO_PATH_NOT_WHITELISTED);
    }

    #[test]
    fn test_empty_path_rejected() {
        let validator = RepoPathValidator::default();
        let result = validator.validate("");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, E_REPO_PATH_INVALID_FORMAT);
        assert!(err.message.contains("cannot be empty"));
    }

    #[test]
    fn test_whitespace_path_rejected() {
        let validator = RepoPathValidator::default();
        let result = validator.validate("   ");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, E_REPO_PATH_INVALID_FORMAT);
    }

    #[test]
    fn test_whitelisted_path_accepted() {
        let validator = RepoPathValidator::new(vec![PathBuf::from("/home")]);
        // Use a path that doesn't need to exist for the test
        let result = validator.validate("/home/user/repo");
        assert!(result.is_ok());
    }

    #[test]
    fn test_non_whitelisted_path_rejected() {
        let validator = RepoPathValidator::new(vec![PathBuf::from("/home")]);
        let result = validator.validate("/etc/config");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, E_REPO_PATH_NOT_WHITELISTED);
        assert!(err.message.contains("not in the whitelist"));
    }

    #[test]
    fn test_empty_whitelist_allows_all() {
        let validator = RepoPathValidator::new(vec![]);
        let result = validator.validate("/any/path");
        assert!(result.is_ok());
    }

    #[test]
    fn test_tmp_path_accepted_by_default() {
        let validator = RepoPathValidator::default();
        let result = validator.validate("/tmp/test-repo");
        assert!(result.is_ok());
    }

    #[test]
    fn test_workspace_path_accepted_by_default() {
        let validator = RepoPathValidator::default();
        let result = validator.validate("/workspace/project");
        assert!(result.is_ok());
    }

    #[test]
    fn test_legitimate_parent_dir_navigation_accepted() {
        let validator = RepoPathValidator::new(vec![PathBuf::from("/home")]);
        // Legitimate path with parent dir that stays within whitelist
        let result = validator.validate("/home/user/../user/repo");
        assert!(result.is_ok());
        // Verify it normalizes to the expected path
        let path = result.unwrap();
        assert_eq!(path, PathBuf::from("/home/user/repo"));
    }

    #[test]
    fn test_traversal_above_root_rejected() {
        let validator = RepoPathValidator::new(vec![PathBuf::from("/home")]);
        // Path that tries to traverse above root - these normalize but may be rejected by whitelist
        let result = validator.validate("/../../etc/passwd");
        assert!(result.is_err());
        let err = result.unwrap_err();
        // This will either be traversal (above root) or whitelist error
        assert!(err.code == E_REPO_PATH_TRAVERSAL || err.code == E_REPO_PATH_NOT_WHITELISTED);
    }
}
