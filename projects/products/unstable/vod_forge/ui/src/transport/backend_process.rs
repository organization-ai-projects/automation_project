use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use crate::diagnostics::UiError;

pub struct BackendProcess {
    pub child: Child,
    pub stdin: ChildStdin,
    pub stdout: ChildStdout,
}

impl BackendProcess {
    pub fn spawn(binary: &str) -> Result<Self, UiError> {
        let mut child = Command::new(binary)
            .arg("serve")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| UiError::Ipc(format!("failed to spawn backend: {}", e)))?;
        let stdin = child.stdin.take().ok_or_else(|| UiError::Ipc("no stdin".to_string()))?;
        let stdout = child.stdout.take().ok_or_else(|| UiError::Ipc("no stdout".to_string()))?;
        Ok(BackendProcess { child, stdin, stdout })
    }
}
