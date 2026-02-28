#![allow(dead_code)]
use crate::diagnostics::error::UiError;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};

pub struct IpcClient {
    child: Child,
    stdin: ChildStdin,
    reader: BufReader<std::process::ChildStdout>,
    next_id: u64,
}

impl IpcClient {
    pub fn new() -> Result<Self, UiError> {
        let backend_bin = std::env::var("SIMKERNEL_BACKEND_BIN")
            .unwrap_or_else(|_| "simkernel_backend".to_string());
        let mut child = Command::new(&backend_bin)
            .arg("serve")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| UiError::BackendSpawn(format!("Failed to spawn {}: {}", backend_bin, e)))?;
        let stdin = child.stdin.take().ok_or(UiError::BackendSpawn("no stdin".to_string()))?;
        let stdout = child.stdout.take().ok_or(UiError::BackendSpawn("no stdout".to_string()))?;
        Ok(Self { child, stdin, reader: BufReader::new(stdout), next_id: 1 })
    }

    pub fn send_request(&mut self, payload: &str) -> Result<String, UiError> {
        let id = self.next_id;
        self.next_id += 1;
        let msg = format!("{{\"id\":{},\"payload\":{}}}\n", id, payload);
        self.stdin.write_all(msg.as_bytes()).map_err(|e| UiError::Ipc(e.to_string()))?;
        let mut line = String::new();
        self.reader.read_line(&mut line).map_err(|e| UiError::Ipc(e.to_string()))?;
        Ok(line.trim().to_string())
    }

    pub fn new_run(&mut self, pack_kind: &str, seed: u64, ticks: u64) -> Result<String, UiError> {
        let payload = serde_json::json!({
            "type": "NewRun",
            "pack_kind": pack_kind,
            "seed": seed,
            "ticks": ticks,
            "turns": 0,
            "ticks_per_turn": 10
        });
        self.send_request(&payload.to_string())
    }

    pub fn shutdown(&mut self) -> Result<(), UiError> {
        let payload = r#"{"type":"Shutdown"}"#;
        let _ = self.send_request(payload);
        let _ = self.child.wait();
        Ok(())
    }
}

impl Drop for IpcClient {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}
