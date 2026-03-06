// projects/products/unstable/protocol_builder/ui/src/transport/backend_process.rs
use crate::diagnostics::ui_error::UiError;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};

pub struct BackendProcess {
    pub child: Child,
    pub stdin: ChildStdin,
    pub reader: BufReader<std::process::ChildStdout>,
}

impl BackendProcess {
    pub fn spawn(binary: &str) -> Result<Self, UiError> {
        let mut child = Command::new(binary)
            .arg("serve")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| UiError::SpawnFailed(e.to_string()))?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| UiError::IpcError("stdin not captured".to_string()))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| UiError::IpcError("stdout not captured".to_string()))?;
        let reader = BufReader::new(stdout);
        Ok(Self {
            child,
            stdin,
            reader,
        })
    }

    pub fn exchange(&mut self, line: &str) -> Result<String, UiError> {
        writeln!(self.stdin, "{line}").map_err(|e| UiError::IpcError(e.to_string()))?;
        self.stdin
            .flush()
            .map_err(|e| UiError::IpcError(e.to_string()))?;

        let mut response = String::new();
        let read = self
            .reader
            .read_line(&mut response)
            .map_err(|e| UiError::IpcError(e.to_string()))?;
        if read == 0 {
            return Err(UiError::IpcError(
                "backend closed IPC stream unexpectedly".to_string(),
            ));
        }
        Ok(response.trim().to_string())
    }
}

impl Drop for BackendProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
