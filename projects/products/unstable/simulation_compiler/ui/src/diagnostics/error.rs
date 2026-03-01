// projects/products/unstable/simulation_compiler/ui/src/diagnostics/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {
    #[error("transport error: {0}")]
    Transport(String),
    #[error("render error: {0}")]
    Render(String),
    #[error("internal error: {0}")]
    Internal(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transport_error_display() {
        let e = UiError::Transport("connection refused".to_string());
        assert!(e.to_string().contains("transport error"));
    }
}
