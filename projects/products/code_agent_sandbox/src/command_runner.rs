// projects/products/code_agent_sandbox/src/command_runner.rs
use std::{
    io::Read,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use anyhow::{bail, Context, Result};
use serde_json::json;
use wait_timeout::ChildExt;

use crate::{actions::ActionResult, policy::Policy, runner_config::RunnerConfig};

#[derive(Clone)]
pub struct CommandRunner {
    pub policy: Policy,
    pub cfg: RunnerConfig,
}

impl CommandRunner {
    pub fn new(policy: Policy, cfg: RunnerConfig) -> Self {
        Self { policy, cfg }
    }

    fn drain(mut r: impl Read + Send + 'static) -> thread::JoinHandle<Vec<u8>> {
        thread::spawn(move || {
            let mut buf = Vec::new();
            let _ = r.read_to_end(&mut buf);
            buf
        })
    }

    pub fn run_cargo(&self, subcommand: &str, args: &[String]) -> Result<ActionResult> {
        if !self.cfg.allowed_bins.iter().any(|b| b == "cargo") {
            bail!("cargo is not allowed by runner config");
        }

        if !self
            .cfg
            .allowed_cargo_subcommands
            .iter()
            .any(|s| s == subcommand)
        {
            return Ok(ActionResult::error(
                "PolicyViolation",
                format!("cargo subcommand not allowed: {subcommand}"),
            ));
        }

        for a in args {
            if a.contains("..")
                || a.contains('\0')
                || a.starts_with("--manifest-path")
                || a.starts_with("--target-dir")
                || a.starts_with("--config")
                || a.starts_with("-Z")
            {
                return Ok(ActionResult::error(
                    "PolicyViolation",
                    format!("suspicious arg: {a}"),
                ));
            }
        }

        let mut cmd = Command::new("cargo");
        cmd.arg(subcommand).arg("--locked").arg("--offline");
        for a in args {
            cmd.arg(a);
        }

        cmd.current_dir(self.policy.work_root());
        cmd.stdin(Stdio::null());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        cmd.env_clear();
        for k in &self.cfg.env_allowlist {
            if let Ok(v) = std::env::var(k) {
                cmd.env(k, v);
            }
        }

        let mut child = cmd.spawn().context("failed to spawn cargo")?;

        let stdout_handle = child.stdout.take().map(Self::drain);
        let stderr_handle = child.stderr.take().map(Self::drain);

        let timeout = Duration::from_millis(self.cfg.timeout_ms);
        let status_opt = child.wait_timeout(timeout).context("wait_timeout failed")?;

        match status_opt {
            Some(status) => {
                let stdout = stdout_handle
                    .map(|h| String::from_utf8_lossy(&h.join().unwrap_or_default()).to_string())
                    .unwrap_or_default();

                let stderr = stderr_handle
                    .map(|h| String::from_utf8_lossy(&h.join().unwrap_or_default()).to_string())
                    .unwrap_or_default();

                let code = status.code().unwrap_or(-1);
                let ok = status.success();
                let kind = format!("Cargo{}", subcommand.to_ascii_uppercase());

                Ok(ActionResult::success(
                    kind,
                    if ok { "ok" } else { "failed" },
                    Some(json!({
                        "exit_code": code,
                        "stdout": truncate(&stdout, 2000),
                        "stderr": truncate(&stderr, 2000)
                    })),
                ))
            }
            None => {
                let _ = child.kill();
                let _ = child.wait();

                let _ = stdout_handle.map(|h| h.join());
                let _ = stderr_handle.map(|h| h.join());

                Ok(ActionResult::error(
                    "Timeout",
                    format!(
                        "cargo {subcommand} timed out after {}ms",
                        self.cfg.timeout_ms
                    ),
                ))
            }
        }
    }

    /// Generic command runner.
    /// Keep it safe: allowlist + env_clear + cwd + timeout.
    /// If you don't need it, delete it (less surface area).
    pub fn run_command(&self, program: &str, args: &[String]) -> Result<ActionResult> {
        if !self.cfg.allowed_bins.iter().any(|b| b == program) {
            return Ok(ActionResult::error(
                "PolicyViolation",
                format!("binary not allowed: {program}"),
            ));
        }

        for a in args {
            if a.contains("..") || a.contains('\0') {
                return Ok(ActionResult::error(
                    "PolicyViolation",
                    format!("suspicious arg: {a}"),
                ));
            }
        }

        let mut cmd = Command::new(program);
        for arg in args {
            cmd.arg(arg);
        }

        cmd.current_dir(self.policy.work_root());
        cmd.stdin(Stdio::null());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        cmd.env_clear();
        for k in &self.cfg.env_allowlist {
            if let Ok(v) = std::env::var(k) {
                cmd.env(k, v);
            }
        }

        let mut child = cmd.spawn().context("failed to spawn command")?;

        let stdout_handle = child.stdout.take().map(Self::drain);
        let stderr_handle = child.stderr.take().map(Self::drain);

        let timeout = Duration::from_millis(self.cfg.timeout_ms);
        let status_opt = child.wait_timeout(timeout).context("wait_timeout failed")?;

        match status_opt {
            Some(status) if status.success() => {
                let stdout = stdout_handle
                    .map(|h| String::from_utf8_lossy(&h.join().unwrap_or_default()).to_string())
                    .unwrap_or_default();

                let stderr = stderr_handle
                    .map(|h| String::from_utf8_lossy(&h.join().unwrap_or_default()).to_string())
                    .unwrap_or_default();

                Ok(ActionResult::success(
                    "CommandExecuted",
                    "ok",
                    Some(json!({
                        "exit_code": status.code().unwrap_or(-1),
                        "stdout": truncate(&stdout, 2000),
                        "stderr": truncate(&stderr, 2000)
                    })),
                ))
            }
            Some(status) => {
                let stdout = stdout_handle
                    .map(|h| String::from_utf8_lossy(&h.join().unwrap_or_default()).to_string())
                    .unwrap_or_default();

                let stderr = stderr_handle
                    .map(|h| String::from_utf8_lossy(&h.join().unwrap_or_default()).to_string())
                    .unwrap_or_default();

                Ok(ActionResult::error(
                    "CommandFailed",
                    format!(
                        "exit_code={} stderr={}",
                        status.code().unwrap_or(-1),
                        truncate(&stderr, 2000)
                    ),
                ))
            }
            None => {
                let _ = child.kill();
                let _ = child.wait();

                let _ = stdout_handle.map(|h| h.join());
                let _ = stderr_handle.map(|h| h.join());

                Ok(ActionResult::error(
                    "Timeout",
                    format!("command timed out after {}ms", self.cfg.timeout_ms),
                ))
            }
        }
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }

    let mut cut = max;
    while cut > 0 && !s.is_char_boundary(cut) {
        cut -= 1;
    }

    let mut t = s[..cut].to_string();
    t.push_str("\n...[truncated]...");
    t
}
