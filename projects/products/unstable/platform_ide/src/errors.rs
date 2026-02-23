// projects/products/unstable/platform_ide/src/errors.rs
use thiserror::Error;

/// The canonical error type for the platform-IDE.
///
/// Error messages never include raw file-system paths or forbidden repository
/// paths. All path-related errors are presented as generic access-denied
/// messages to prevent information leakage through error channels.
#[derive(Debug, Error)]
pub enum IdeError {
    #[error("authentication failed")]
    AuthFailed,

    #[error("session token is missing or expired")]
    SessionExpired,

    #[error("access denied")]
    PermissionDenied,

    #[error("the requested issue is not visible to the current user")]
    IssueNotVisible,

    #[error("the path is not permitted by the slice manifest")]
    PathNotAllowed,

    #[error("no slice is loaded for the current issue")]
    NoSliceLoaded,

    #[error("the file buffer is not open")]
    BufferNotOpen,

    #[error("no changes are staged in the change set")]
    EmptyChangeSet,

    #[error("platform API error: {code}")]
    ApiError {
        /// Stable machine-readable error code from the platform.
        code: String,
    },

    #[error("platform returned an unexpected response")]
    UnexpectedResponse,

    #[error("offline mode is not permitted by platform policy")]
    OfflineNotPermitted,

    #[error("network error communicating with the platform")]
    Network(#[from] reqwest::Error),
}
