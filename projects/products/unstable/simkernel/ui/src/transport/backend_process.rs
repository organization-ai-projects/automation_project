#![allow(dead_code)]
use crate::diagnostics::error::UiError;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

pub struct BackendProcess {
    child: Child,
    pub stdin: ChildStdin,
    pub stdout: ChildStdout,
}

impl BackendProcess {
    pub fn spawn(backend_bin: &str, pack_kind: Option<&str>) -> Result<Self, UiError> {
        let mut cmd = Command::new(backend_bin);
        cmd.arg("serve");
        if let Some(pack) = pack_kind {
            cmd.args(["--pack", pack]);
        }
        cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::inherit());
        let mut child = cmd.spawn().map_err(|e| UiError::BackendSpawn(e.to_string()))?;
        let stdin = child.stdin.take().ok_or(UiError::BackendSpawn("no stdin".to_string()))?;
        let stdout = child.stdout.take().ok_or(UiError::BackendSpawn("no stdout".to_string()))?;
        Ok(Self { child, stdin, stdout })
    }

    pub fn kill(&mut self) {
        let _ = self.child.kill();
    }
}
