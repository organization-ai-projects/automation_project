// projects/products/stable/varina/backend/src/repo_path_validator.rs
use std::path::{Path, PathBuf};

/// Error codes for repo path validation failures
pub const E_REPO_PATH_INVALID_FORMAT: i32 = 1500;
pub const E_REPO_PATH_NOT_WHITELISTED: i32 = 1501;
pub const E_REPO_PATH_TRAVERSAL: i32 = 1502;

/// Result type for repo path validation
pub type ValidationResult<T> = Result<T, ValidationError>;

/// Validation error with code and message
#[derive(Debug, Clone)]
pub struct ValidationError {
    #[allow(dead_code)]
    pub code: i32,
    pub message: String,
}

impl ValidationError {
    pub fn new(code: i32, message: String) -> Self {
        Self { code, message }
    }
}

/// Repository path validator with whitelist support
pub struct RepoPathValidator {
    whitelist: Vec<PathBuf>,
}

impl RepoPathValidator {
    /// Create a new validator with the given whitelist of allowed paths
    #[allow(dead_code)]
    pub fn new(whitelist: Vec<PathBuf>) -> Self {
        Self { whitelist }
    }

    /// Create a validator with default whitelist
    pub fn default() -> Self {
        // Default whitelist allows common development directories
        let whitelist = vec![
            PathBuf::from("/home"),
            PathBuf::from("/tmp"),
            PathBuf::from("/workspace"),
        ];
        Self { whitelist }
    }

    /// Validate and normalize a repository path
    pub fn validate(&self, repo_path: &str) -> ValidationResult<PathBuf> {
        // Check for path traversal attempts
        if repo_path.contains("..") {
            return Err(ValidationError::new(
                E_REPO_PATH_TRAVERSAL,
                "Path traversal detected: '..' not allowed in repository path".to_string(),
            ));
        }

        // Check for empty or whitespace-only paths
        if repo_path.trim().is_empty() {
            return Err(ValidationError::new(
                E_REPO_PATH_INVALID_FORMAT,
                "Repository path cannot be empty".to_string(),
            ));
        }

        // Normalize the path
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

        // Canonicalize if the path exists
        let canonical = if normalized.exists() {
            normalized.canonicalize().map_err(|e| {
                ValidationError::new(
                    E_REPO_PATH_INVALID_FORMAT,
                    format!("Failed to canonicalize path: {}", e),
                )
            })?
        } else {
            normalized
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

    /// Check if a path is within any whitelisted directory
    fn is_whitelisted(&self, path: &Path) -> bool {
        if self.whitelist.is_empty() {
            // If whitelist is empty, allow all paths
            return true;
        }

        self.whitelist.iter().any(|allowed| {
            path.starts_with(allowed) || allowed.starts_with(path)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_traversal_rejected() {
        let validator = RepoPathValidator::default();
        let result = validator.validate("/home/user/../etc/passwd");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, E_REPO_PATH_TRAVERSAL);
        assert!(err.message.contains("Path traversal"));
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
}
