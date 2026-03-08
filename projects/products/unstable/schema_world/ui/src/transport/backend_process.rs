use crate::diagnostics::error::UiError;
use std::io::{BufReader, BufWriter};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

pub struct BackendProcess {
    _child: Child,
    pub stdin: BufWriter<ChildStdin>,
    pub stdout: BufReader<ChildStdout>,
}

impl BackendProcess {
    pub fn spawn(binary: &str) -> Result<Self, UiError> {
        let mut child = Command::new(binary)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| UiError::SpawnFailed(e.to_string()))?;

        let child_stdin = child
            .stdin
            .take()
            .ok_or_else(|| UiError::SpawnFailed("missing child stdin".to_string()))?;
        let child_stdout = child
            .stdout
            .take()
            .ok_or_else(|| UiError::SpawnFailed("missing child stdout".to_string()))?;

        Ok(Self {
            _child: child,
            stdin: BufWriter::new(child_stdin),
            stdout: BufReader::new(child_stdout),
        })
    }
}
