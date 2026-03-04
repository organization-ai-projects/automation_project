// projects/products/unstable/evolutionary_system_generator/ui/src/transport/ipc_client.rs
use std::io::{BufRead, BufReader, Write};
use std::process::{ChildStdin, ChildStdout};

use crate::diagnostics::ui_error::UiError;

pub struct IpcClient {
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl IpcClient {
    pub fn new(stdin: ChildStdin, stdout: ChildStdout) -> Self {
        Self {
            stdin,
            stdout: BufReader::new(stdout),
        }
    }

    pub fn send_request(&mut self, request: &str) -> Result<String, UiError> {
        self.stdin
            .write_all(request.as_bytes())
            .map_err(|e| UiError::Ipc(e.to_string()))?;
        self.stdin
            .write_all(b"\n")
            .map_err(|e| UiError::Ipc(e.to_string()))?;
        self.stdin
            .flush()
            .map_err(|e| UiError::Ipc(e.to_string()))?;

        let mut response = String::new();
        let read = self
            .stdout
            .read_line(&mut response)
            .map_err(|e| UiError::Ipc(e.to_string()))?;
        if read == 0 {
            return Err(UiError::Ipc(
                "backend closed stdout before response".to_string(),
            ));
        }
        Ok(response.trim().to_string())
    }
}
