use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {
    #[error("usage: space_diplo_wars_ui <run|replay|snapshot|validate> [args]")]
    Usage,
    #[error("failed to spawn backend binary '{0}': {1}")]
    BackendSpawnFailed(String, String),
    #[error("backend exited with code {0}")]
    BackendExit(i32),
}

impl UiError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Usage => 2,
            Self::BackendSpawnFailed(_, _) => 5,
            Self::BackendExit(code) => *code,
        }
    }
}
