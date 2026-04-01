#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[allow(dead_code)]
    #[error("verification failed: {0}")]
    VerificationFailed(String),

    #[error("invalid usage: {0}")]
    InvalidUsage(String),

    #[error("manifest/format error: {0}")]
    ManifestFormat(String),

    #[error("internal error: {0}")]
    Internal(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

impl Error {
    pub fn exit_code(&self) -> i32 {
        match self {
            Error::VerificationFailed(_) => 1,
            Error::InvalidUsage(_) => 2,
            Error::ManifestFormat(_) => 3,
            Error::Internal(_) | Error::Io(_) => 4,
        }
    }
}
