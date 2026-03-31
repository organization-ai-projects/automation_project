use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EditorError {
    #[error("empty source code")]
    EmptySource,
    #[error("missing .rhl content")]
    InvalidContent(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorState {
    pub source_code: String,
    pub transpiled_output: Option<String>,
    pub error_message: Option<String>,
}

impl EditorState {
    pub fn new(source_code: String) -> Self {
        Self {
            source_code,
            transpiled_output: None,
            error_message: None,
        }
    }

    pub fn validate(&self) -> Result<(), EditorError> {
        if self.source_code.trim().is_empty() {
            return Err(EditorError::EmptySource);
        }
        Ok(())
    }

    pub fn set_transpiled(&mut self, output: String) {
        self.transpiled_output = Some(output);
        self.error_message = None;
    }

    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
        self.transpiled_output = None;
    }
}
