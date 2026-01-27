// projects/libraries/security/src/permission_error.rs
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum PermissionError {
    #[error("unauthorized: insufficient permissions")]
    Unauthorized,
}
