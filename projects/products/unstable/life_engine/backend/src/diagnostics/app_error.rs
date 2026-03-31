use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Config(String),
    Process(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(msg) => write!(f, "config error: {msg}"),
            AppError::Process(msg) => write!(f, "process error: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}
