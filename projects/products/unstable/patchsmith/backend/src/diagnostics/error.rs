use thiserror::Error;

#[derive(Debug, Error)]
pub enum PatchsmithError {
    #[error("parse error: {0}")]
    Parse(String),

    #[error("apply error: {0}")]
    Apply(String),

    #[error("verify error: {0}")]
    Verify(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("internal error: {0}")]
    Internal(String),
}

impl PatchsmithError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Verify(_) => 1,
            Self::Parse(_) => 3,
            Self::Apply(_) => 3,
            Self::Io(_) => 4,
            Self::Internal(_) => 4,
        }
    }
}
