use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {
    #[error("usage: universe_simulation_engine_ui run|convert [args]")]
    Usage,
    #[error("backend command failed with code {0}")]
    Backend(i32),
    #[error("io error: {0}")]
    Io(String),
}

impl UiError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Usage => 2,
            Self::Backend(code) => *code,
            Self::Io(_) => 5,
        }
    }
}
