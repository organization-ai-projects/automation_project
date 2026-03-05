// projects/products/unstable/colony_manager/ui/src/ui_error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {
    #[error("usage: colony_manager_ui run|replay [args]")]
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
