use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum IpcError {
    #[error("process error: {0}")]
    Process(String),

    #[error("io error: {0}")]
    Io(String),

    #[error("protocol error: {0}")]
    Protocol(String),
}

pub struct IpcClient {
    child: Child,
    next_id: u64,
}

impl IpcClient {
    pub fn spawn(scenario: &Path) -> Result<Self, IpcError> {
        let backend_bin = std::env::var("MARKET_TYCOON_BACKEND_BIN")
            .unwrap_or_else(|_| "market_tycoon_backend".to_string());

        let child = Command::new(&backend_bin)
            .arg("serve")
            .arg("--scenario")
            .arg(scenario)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| IpcError::Process(e.to_string()))?;

        Ok(Self { child, next_id: 1 })
    }

    pub fn send_raw(&mut self, request_json: &str) -> Result<String, IpcError> {
        let stdin = self
            .child
            .stdin
            .as_mut()
            .ok_or_else(|| IpcError::Io("stdin unavailable".into()))?;
        writeln!(stdin, "{request_json}").map_err(|e| IpcError::Io(e.to_string()))?;
        stdin.flush().map_err(|e| IpcError::Io(e.to_string()))?;

        let stdout = self
            .child
            .stdout
            .as_mut()
            .ok_or_else(|| IpcError::Io("stdout unavailable".into()))?;
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .map_err(|e| IpcError::Io(e.to_string()))?;
        Ok(line)
    }

    pub fn new_run(&mut self, seed: u64, ticks: u64) -> Result<String, IpcError> {
        let id = self.next_id;
        self.next_id += 1;
        let msg = format!(
            r#"{{"id":{id},"request":{{"type":"NewRun","seed":{seed},"ticks":{ticks}}}}}"#
        );
        self.send_raw(&msg)
    }

    pub fn run_to_end(&mut self) -> Result<String, IpcError> {
        let id = self.next_id;
        self.next_id += 1;
        let msg = format!(r#"{{"id":{id},"request":{{"type":"RunToEnd"}}}}"#);
        self.send_raw(&msg)
    }

    pub fn get_report(&mut self) -> Result<String, IpcError> {
        let id = self.next_id;
        self.next_id += 1;
        let msg = format!(r#"{{"id":{id},"request":{{"type":"GetReport"}}}}"#);
        self.send_raw(&msg)
    }

    pub fn shutdown(mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

impl Drop for IpcClient {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
