use crate::diagnostics::error::UiError;
use std::io::{BufRead, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

pub struct BackendProcess {
    pub child: Child,
    pub stdin: ChildStdin,
    pub stdout: std::io::BufReader<ChildStdout>,
}

impl BackendProcess {
    pub fn spawn(backend_bin: &str) -> Result<Self, UiError> {
        let mut child = Command::new(backend_bin)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| UiError::Transport(e.to_string()))?;
        let stdin = child.stdin.take().ok_or_else(|| UiError::Transport("no stdin".to_string()))?;
        let stdout = child.stdout.take().ok_or_else(|| UiError::Transport("no stdout".to_string()))?;
        Ok(Self {
            child,
            stdin,
            stdout: std::io::BufReader::new(stdout),
        })
    }

    pub fn send_line(&mut self, line: &str) -> Result<(), UiError> {
        writeln!(self.stdin, "{}", line).map_err(|e| UiError::Transport(e.to_string()))
    }

    pub fn recv_line(&mut self) -> Result<String, UiError> {
        let mut line = String::new();
        self.stdout.read_line(&mut line).map_err(|e| UiError::Transport(e.to_string()))?;
        Ok(line.trim_end().to_string())
    }
}
