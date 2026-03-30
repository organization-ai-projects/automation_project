use std::fmt;

#[derive(Debug)]
pub enum UiError {
    Process(String),
}

impl fmt::Display for UiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UiError::Process(msg) => write!(f, "ui error: {msg}"),
        }
    }
}

impl std::error::Error for UiError {}
