use crate::diagnostics::ui_error::UiError;
use std::process::Command;

pub struct BackendProcess {
    backend_bin: String,
}

impl BackendProcess {
    pub fn new() -> Self {
        let backend_bin = std::env::var("CITY_BUILDER_BACKEND_BIN")
            .unwrap_or_else(|_| "city_builder_backend".to_string());
        Self { backend_bin }
    }

    pub fn backend_bin(&self) -> &str {
        &self.backend_bin
    }

    pub fn run(&self, args: &[String]) -> Result<i32, UiError> {
        let status = Command::new(&self.backend_bin)
            .args(args)
            .status()
            .map_err(|e| UiError::Io(e.to_string()))?;
        Ok(status.code().unwrap_or(5))
    }
}
