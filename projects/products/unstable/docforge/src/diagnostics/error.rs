use crate::model::block_id::BlockId;

#[derive(Debug, thiserror::Error)]
pub enum DocError {
    #[error("block not found: {0:?}")]
    BlockNotFound(BlockId),
    #[error("invalid operation: {0}")]
    InvalidOperation(String),
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("IO error: {0}")]
    Io(String),
    #[error("snapshot checksum mismatch")]
    ChecksumMismatch,
    #[error("replay error: {0}")]
    Replay(String),
}

impl From<std::io::Error> for DocError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}
