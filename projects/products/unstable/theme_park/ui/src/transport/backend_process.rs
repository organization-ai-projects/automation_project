#![allow(dead_code)]
use crate::diagnostics::error::UiError;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

/// Manages the backend child process lifecycle.
pub struct BackendProcess {
    pub child: Child,
    pub stdin: ChildStdin,
    pub stdout: ChildStdout,
}

impl BackendProcess {
    pub fn spawn(backend_bin: &str, scenario: Option<&str>) -> Result<Self, UiError> {
        let mut cmd = Command::new(backend_bin);
        cmd.arg("serve");
        if let Some(s) = scenario {
            cmd.args(["--scenario", s]);
        }
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit());
        let mut child = cmd
            .spawn()
            .map_err(|e| UiError::BackendSpawn(format!("Failed to spawn {}: {}", backend_bin, e)))?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| UiError::BackendSpawn("no stdin".to_string()))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| UiError::BackendSpawn("no stdout".to_string()))?;
        Ok(Self {
            child,
            stdin,
            stdout,
        })
    }

    pub fn kill(&mut self) {
        let _ = self.child.kill();
    }
}
