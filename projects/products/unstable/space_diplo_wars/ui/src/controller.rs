use std::process::Command;

use crate::ui_error::UiError;

pub struct Controller;

impl Controller {
    pub fn run_command(command: &str, args: &[String]) -> Result<(), UiError> {
        match command {
            "run" | "replay" | "snapshot" | "validate" => Self::forward_to_backend(command, args),
            _ => Err(UiError::Usage),
        }
    }

    fn forward_to_backend(command: &str, args: &[String]) -> Result<(), UiError> {
        let backend_bin = std::env::var("SPACE_DIPLO_WARS_BACKEND_BIN")
            .unwrap_or_else(|_| "space_diplo_wars_backend".to_string());

        let mut cmd = Command::new(&backend_bin);
        cmd.arg(command);
        cmd.args(args);

        let status = cmd
            .status()
            .map_err(|e| UiError::BackendSpawnFailed(backend_bin, e.to_string()))?;

        if status.success() {
            Ok(())
        } else {
            Err(UiError::BackendExit(status.code().unwrap_or(5)))
        }
    }
}
