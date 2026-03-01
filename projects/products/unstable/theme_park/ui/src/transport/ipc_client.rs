#![allow(dead_code)]
use crate::diagnostics::error::UiError;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};

/// JSON Lines IPC client that speaks to the backend process.
pub struct IpcClient {
    child: Child,
    stdin: ChildStdin,
    reader: BufReader<std::process::ChildStdout>,
    next_id: u64,
}

impl IpcClient {
    pub fn new(scenario: Option<&str>) -> Result<Self, UiError> {
        let backend_bin = std::env::var("THEME_PARK_BACKEND_BIN")
            .unwrap_or_else(|_| "theme_park_backend".to_string());
        let mut cmd = Command::new(&backend_bin);
        cmd.arg("serve");
        if let Some(s) = scenario {
            cmd.args(["--scenario", s]);
        }
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit());
        let mut child = cmd.spawn().map_err(|e| {
            UiError::BackendSpawn(format!("Failed to spawn {}: {}", backend_bin, e))
        })?;
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
            reader: BufReader::new(stdout),
            next_id: 1,
        })
    }

    /// Send a request payload and return the raw response line.
    pub fn send(&mut self, payload: serde_json::Value) -> Result<serde_json::Value, UiError> {
        let id = self.next_id;
        self.next_id += 1;
        let msg = serde_json::json!({ "id": id, "payload": payload });
        let line =
            serde_json::to_string(&msg).map_err(|e| UiError::Serialization(e.to_string()))?;
        self.stdin
            .write_all(format!("{}\n", line).as_bytes())
            .map_err(|e| UiError::Ipc(e.to_string()))?;
        let mut resp_line = String::new();
        self.reader
            .read_line(&mut resp_line)
            .map_err(|e| UiError::Ipc(e.to_string()))?;
        let resp: serde_json::Value = serde_json::from_str(resp_line.trim())
            .map_err(|e| UiError::Serialization(e.to_string()))?;
        self.check_error(&resp)?;
        Ok(resp)
    }

    pub fn ping(&mut self) -> Result<(), UiError> {
        self.send(serde_json::json!({ "type": "Ping" }))?;
        Ok(())
    }

    pub fn load_scenario(&mut self, path: &str) -> Result<(), UiError> {
        self.send(serde_json::json!({ "type": "LoadScenario", "path": path }))?;
        Ok(())
    }

    pub fn new_run(&mut self, seed: u64, ticks: u64) -> Result<(), UiError> {
        self.send(serde_json::json!({ "type": "NewRun", "seed": seed, "ticks": ticks }))?;
        Ok(())
    }

    /// Run to completion and return the report JSON string.
    pub fn run_to_end(&mut self) -> Result<String, UiError> {
        let resp = self.send(serde_json::json!({ "type": "RunToEnd" }))?;
        let report_json = resp
            .get("report_json")
            .ok_or_else(|| UiError::Ipc("missing report_json".to_string()))?;
        serde_json::to_string_pretty(report_json).map_err(|e| UiError::Serialization(e.to_string()))
    }

    pub fn get_report(&mut self) -> Result<String, UiError> {
        let resp = self.send(serde_json::json!({ "type": "GetReport" }))?;
        let report_json = resp
            .get("report_json")
            .ok_or_else(|| UiError::Ipc("missing report_json".to_string()))?;
        serde_json::to_string_pretty(report_json).map_err(|e| UiError::Serialization(e.to_string()))
    }

    pub fn save_replay(&mut self, path: &str) -> Result<(), UiError> {
        self.send(serde_json::json!({ "type": "SaveReplay", "path": path }))?;
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<(), UiError> {
        let _ = self.send(serde_json::json!({ "type": "Shutdown" }));
        let _ = self.child.wait();
        Ok(())
    }

    fn check_error(&self, resp: &serde_json::Value) -> Result<(), UiError> {
        if resp.get("type").and_then(|t| t.as_str()) == Some("Error") {
            let code = resp
                .get("code")
                .and_then(|c| c.as_str())
                .unwrap_or("UNKNOWN");
            let message = resp.get("message").and_then(|m| m.as_str()).unwrap_or("");
            return Err(UiError::Ipc(format!("{}: {}", code, message)));
        }
        Ok(())
    }
}

impl Drop for IpcClient {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}
