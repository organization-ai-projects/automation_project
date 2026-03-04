use crate::diagnostics::ui_error::UiError;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

pub struct BackendProcess {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl BackendProcess {
    pub fn spawn(backend_bin: &str) -> Result<Self, UiError> {
        let mut child = Command::new(backend_bin)
            .arg("serve")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|error| UiError::Transport(error.to_string()))?;

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| UiError::Transport("backend stdin unavailable".to_string()))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| UiError::Transport("backend stdout unavailable".to_string()))?;

        Ok(Self {
            child,
            stdin,
            stdout: BufReader::new(stdout),
        })
    }

    pub fn send_line(&mut self, line: &str) -> Result<(), UiError> {
        writeln!(&mut self.stdin, "{line}")
            .map_err(|error| UiError::Transport(error.to_string()))?;
        self.stdin
            .flush()
            .map_err(|error| UiError::Transport(error.to_string()))
    }

    pub fn read_line(&mut self) -> Result<String, UiError> {
        let mut line = String::new();
        let count = self
            .stdout
            .read_line(&mut line)
            .map_err(|error| UiError::Transport(error.to_string()))?;
        if count == 0 {
            return Err(UiError::Transport("backend closed stdout".to_string()));
        }
        Ok(line)
    }

    pub fn shutdown(mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
