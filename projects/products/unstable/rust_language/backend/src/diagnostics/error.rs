use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid CLI: {0}")]
    InvalidCli(String),

    #[error("lexer error at line {line}, col {col}: {message}")]
    Lexer {
        line: usize,
        col: usize,
        message: String,
    },

    #[error("parser error: {message}")]
    Parser { message: String },

    #[error("transpilation error: {0}")]
    Transpilation(String),

    #[error("ron error: {0}")]
    Ron(String),

    #[error("binary error: {0}")]
    Binary(String),

    #[error("ai error: {0}")]
    Ai(String),

    #[error("io error: {0}")]
    Io(String),
}

impl From<common_ron::RonIoError> for Error {
    fn from(e: common_ron::RonIoError) -> Self {
        Error::Ron(e.to_string())
    }
}

impl From<common_binary::BinaryError> for Error {
    fn from(e: common_binary::BinaryError) -> Self {
        Error::Binary(e.to_string())
    }
}

impl From<ai::AiError> for Error {
    fn from(e: ai::AiError) -> Self {
        Error::Ai(e.to_string())
    }
}
