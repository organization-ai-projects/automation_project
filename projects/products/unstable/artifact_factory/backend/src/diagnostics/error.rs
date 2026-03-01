use thiserror::Error;

#[derive(Debug, Error)]
pub enum FactoryError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("codec error: {0}")]
    Codec(String),
    #[error("analysis error: {0}")]
    Analysis(String),
    #[error("render error: {0}")]
    Render(String),
    #[error("bundle error: {0}")]
    Bundle(String),
}
