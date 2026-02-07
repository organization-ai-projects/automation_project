// projects/products/stable/varina/backend/src/validation_error.rs

/// Error codes for repo path validation failures
pub const E_REPO_PATH_INVALID_FORMAT: i32 = 1500;
pub const E_REPO_PATH_NOT_WHITELISTED: i32 = 1501;
pub const E_REPO_PATH_TRAVERSAL: i32 = 1502;

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
