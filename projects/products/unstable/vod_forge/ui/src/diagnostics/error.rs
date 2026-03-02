#[derive(Debug, thiserror::Error)]
pub enum UiError {
    #[error("ipc error: {0}")]
    Ipc(String),
    #[error("fixture error: {0}")]
    Fixture(String),
    #[error("io error: {0}")]
    Io(String),
}
