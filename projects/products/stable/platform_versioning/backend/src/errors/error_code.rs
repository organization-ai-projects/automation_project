// projects/products/stable/platform_versioning/backend/src/errors/error_code.rs
use serde::{Deserialize, Serialize};

/// Stable machine-readable error codes.
///
/// Codes are part of the public API contract. Variants may be added in minor
/// releases; existing variants must not be removed or renamed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    /// Generic invalid identifier.
    InvalidId,
    /// Path rejected due to traversal or unsafe characters.
    UnsafePath,
    /// The requested repository does not exist.
    RepoNotFound,
    /// The requested object does not exist in the store.
    ObjectNotFound,
    /// The requested ref does not exist.
    RefNotFound,
    /// The requested commit does not exist.
    CommitNotFound,
    /// The stored object failed integrity validation on read.
    CorruptObject,
    /// A write could not be completed atomically.
    AtomicWriteFailed,
    /// The ref update is not a fast-forward and no force flag was given.
    NonFastForward,
    /// A merge conflict was encountered.
    MergeConflict,
    /// Authentication token is missing or invalid.
    AuthRequired,
    /// The caller lacks the required permission.
    PermissionDenied,
    /// The encoding format version is not supported.
    UnsupportedVersion,
    /// The requested issue does not exist.
    IssueNotFound,
    /// A slice projection could not be built.
    SliceBuildFailed,
    /// An unexpected internal error.
    Internal,
}
