// projects/products/unstable/spec_to_runtime_compiler/ui/src/transport/backend_process.rs
use crate::diagnostics::error::UiError;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};

pub struct BackendProcess {
    pub binary_path: String,
    child: Child,
    child_stdin: ChildStdin,
    child_stdout: BufReader<std::process::ChildStdout>,
}

impl BackendProcess {
    pub fn new(binary_path: impl Into<String>) -> Result<Self, UiError> {
        let binary_path = binary_path.into();
        let mut child = Command::new(&binary_path)
            .arg("serve")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| UiError::Transport(format!("failed to spawn backend process: {e}")))?;

        let child_stdin = child
            .stdin
            .take()
            .ok_or_else(|| UiError::Transport("failed to capture backend stdin".to_string()))?;
        let child_stdout = child
            .stdout
            .take()
            .ok_or_else(|| UiError::Transport("failed to capture backend stdout".to_string()))?;

        Ok(Self {
            binary_path,
            child,
            child_stdin,
            child_stdout: BufReader::new(child_stdout),
        })
    }

    pub fn exchange(&mut self, request_json: &str) -> Result<String, UiError> {
        writeln!(self.child_stdin, "{request_json}")
            .map_err(|e| UiError::Transport(format!("failed to write IPC request: {e}")))?;
        self.child_stdin
            .flush()
            .map_err(|e| UiError::Transport(format!("failed to flush IPC request: {e}")))?;

        let mut response_line = String::new();
        let read = self
            .child_stdout
            .read_line(&mut response_line)
            .map_err(|e| UiError::Transport(format!("failed to read IPC response: {e}")))?;
        if read == 0 {
            return Err(UiError::Transport(
                "backend closed IPC stream unexpectedly".to_string(),
            ));
        }
        Ok(response_line.trim().to_string())
    }
}

impl Drop for BackendProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
