// projects/products/unstable/platform_versioning/backend/src/errors/pv_error.rs
use thiserror::Error;

use crate::errors::{ErrorCategory, ErrorCode};

/// The canonical error type for the platform-versioning backend.
///
/// Every public API returns `Result<_, PvError>`. The variants map to a stable
/// [`ErrorCode`] and [`ErrorCategory`] pair so callers can react programmatically
/// without parsing message strings.
#[derive(Debug, Error)]
pub enum PvError {
    #[error("invalid identifier: {0}")]
    InvalidId(String),

    #[error("unsafe path rejected: {0}")]
    UnsafePath(String),

    #[error("repository not found: {0}")]
    RepoNotFound(String),

    #[error("object not found: {0}")]
    ObjectNotFound(String),

    #[error("ref not found: {0}")]
    RefNotFound(String),

    #[error("commit not found: {0}")]
    CommitNotFound(String),

    #[error("corrupt object: {0}")]
    CorruptObject(String),

    #[error("atomic write failed: {0}")]
    AtomicWriteFailed(String),

    #[error("non fast-forward update rejected: {0}")]
    NonFastForward(String),

    #[error("merge conflict: {0}")]
    MergeConflict(String),

    #[error("authentication required: {0}")]
    AuthRequired(String),

    #[error("permission denied: {0}")]
    PermissionDenied(String),

    #[error("unsupported encoding version: {0}")]
    UnsupportedVersion(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("internal error: {0}")]
    Internal(String),
}

impl PvError {
    /// Returns the stable error code for this error.
    pub fn code(&self) -> ErrorCode {
        match self {
            Self::InvalidId(_) => ErrorCode::InvalidId,
            Self::UnsafePath(_) => ErrorCode::UnsafePath,
            Self::RepoNotFound(_) => ErrorCode::RepoNotFound,
            Self::ObjectNotFound(_) => ErrorCode::ObjectNotFound,
            Self::RefNotFound(_) => ErrorCode::RefNotFound,
            Self::CommitNotFound(_) => ErrorCode::CommitNotFound,
            Self::CorruptObject(_) => ErrorCode::CorruptObject,
            Self::AtomicWriteFailed(_) => ErrorCode::AtomicWriteFailed,
            Self::NonFastForward(_) => ErrorCode::NonFastForward,
            Self::MergeConflict(_) => ErrorCode::MergeConflict,
            Self::AuthRequired(_) => ErrorCode::AuthRequired,
            Self::PermissionDenied(_) => ErrorCode::PermissionDenied,
            Self::UnsupportedVersion(_) => ErrorCode::UnsupportedVersion,
            Self::Io(_) => ErrorCode::Internal,
            Self::Internal(_) => ErrorCode::Internal,
        }
    }

    /// Returns the stable error category for this error.
    pub fn category(&self) -> ErrorCategory {
        match self {
            Self::InvalidId(_) | Self::UnsafePath(_) => ErrorCategory::InvalidInput,
            Self::RepoNotFound(_)
            | Self::ObjectNotFound(_)
            | Self::RefNotFound(_)
            | Self::CommitNotFound(_) => ErrorCategory::NotFound,
            Self::CorruptObject(_) => ErrorCategory::Integrity,
            Self::AtomicWriteFailed(_) | Self::Io(_) => ErrorCategory::Io,
            Self::NonFastForward(_) | Self::MergeConflict(_) => ErrorCategory::Conflict,
            Self::AuthRequired(_) => ErrorCategory::Unauthenticated,
            Self::PermissionDenied(_) => ErrorCategory::Unauthorized,
            Self::UnsupportedVersion(_) | Self::Internal(_) => ErrorCategory::Internal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_id_has_correct_code() {
        let err = PvError::InvalidId("bad".to_string());
        assert_eq!(err.code(), ErrorCode::InvalidId);
        assert_eq!(err.category(), ErrorCategory::InvalidInput);
    }

    #[test]
    fn not_found_category() {
        let err = PvError::ObjectNotFound("abc".to_string());
        assert_eq!(err.category(), ErrorCategory::NotFound);
    }

    #[test]
    fn io_error_maps_to_internal_code() {
        let io = std::io::Error::new(std::io::ErrorKind::Other, "disk full");
        let err = PvError::from(io);
        assert_eq!(err.code(), ErrorCode::Internal);
        assert_eq!(err.category(), ErrorCategory::Io);
    }
}
