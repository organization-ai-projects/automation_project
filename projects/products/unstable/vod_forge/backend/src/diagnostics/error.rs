#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error("catalog error: {0}")]
    Catalog(String),
    #[error("packaging error: {0}")]
    Packaging(String),
    #[error("playback error: {0}")]
    Playback(String),
    #[error("io error: {0}")]
    Io(String),
    #[error("codec error: {0}")]
    Codec(String),
}
