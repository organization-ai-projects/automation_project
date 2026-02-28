use thiserror::Error;

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("I/O error: {0}")]
    Io(String),
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("runtime error: {0}")]
    Runtime(String),
    #[error("patch error: {0}")]
    Patch(String),
    #[error("verify error: {0}")]
    Verify(String),
}

impl From<std::io::Error> for AgentError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}

impl From<serde_json::Error> for AgentError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serialization(e.to_string())
    }
}

impl From<runtime_core::RuntimeError> for AgentError {
    fn from(e: runtime_core::RuntimeError) -> Self {
        Self::Runtime(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn io_error_displays() {
        let e = AgentError::Io("not found".to_string());
        assert_eq!(e.to_string(), "I/O error: not found");
    }

    #[test]
    fn patch_error_displays() {
        let e = AgentError::Patch("bad patch".to_string());
        assert_eq!(e.to_string(), "patch error: bad patch");
    }

    #[test]
    fn from_runtime_error() {
        let re = runtime_core::RuntimeError::CyclicGraph;
        let ae = AgentError::from(re);
        assert!(ae.to_string().contains("cyclic"));
    }
}
