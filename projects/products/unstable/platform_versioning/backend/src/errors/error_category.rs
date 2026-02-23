// projects/products/unstable/platform_versioning/backend/src/errors/error_category.rs
use serde::{Deserialize, Serialize};

/// Stable top-level category for a [`super::PvError`].
///
/// Categories are part of the public API contract and must not be renamed once
/// published. Adding new variants is non-breaking; removing or renaming is breaking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCategory {
    /// The caller supplied invalid input (bad id format, unsafe path, etc.).
    InvalidInput,
    /// The requested object, ref, or repository was not found.
    NotFound,
    /// The operation violates a storage integrity constraint.
    Integrity,
    /// An I/O error occurred while reading or writing persistent state.
    Io,
    /// The operation would conflict with existing state (e.g. merge conflict).
    Conflict,
    /// The caller is not authenticated.
    Unauthenticated,
    /// The caller does not have permission to perform the operation.
    Unauthorized,
    /// An internal error occurred that is not the caller's fault.
    Internal,
}
