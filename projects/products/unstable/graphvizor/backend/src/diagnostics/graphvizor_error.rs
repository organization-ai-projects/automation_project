/// Errors produced by the graphvizor tool.
#[derive(Debug, thiserror::Error)]
pub enum GraphvizorError {
    #[error("invalid config: {0}")]
    InvalidConfig(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json parse error: {0}")]
    JsonParse(String),

    #[error("unknown layout: {0}")]
    UnknownLayout(String),
}
