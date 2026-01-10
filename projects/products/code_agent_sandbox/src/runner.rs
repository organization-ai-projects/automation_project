// projects/products/code_agent_sandbox/src/runner.rs
use std::{
    process::{Command, Stdio},
    time::Duration,
};

use anyhow::{bail, Context, Result};
use serde_json::json;
use wait_timeout::ChildExt;

use crate::{actions::ActionResult, policy::Policy};

#[derive(Debug, Clone)]
pub struct RunnerConfig {
    pub allowed_bins: Vec<String>,
    pub allowed_cargo_subcommands: Vec<String>,
    pub timeout_ms: u64,
    pub env_allowlist: Vec<String>,
}

#[derive(Clone)]
pub struct CommandRunner {
    policy: Policy,
    cfg: RunnerConfig,
}

impl CommandRunner {
    pub fn new(policy: Policy, cfg: RunnerConfig) -> Self {
        Self { policy, cfg }
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

        // Hard safety: forbid dangerous args patterns.
        for a in args {
            if a.contains("..") || a.contains('\0') {
                return Ok(ActionResult::error(
                    "PolicyViolation",
                    format!("suspicious arg: {a}"),
                ));
            }
        }

        let mut cmd = Command::new("/home/bezotremi/.cargo/bin/cargo"); // Chemin explicite vers cargo
        cmd.arg(subcommand);
        for a in args {
            cmd.arg(a);
        }

        cmd.current_dir(self.policy.repo_root());
        cmd.stdin(Stdio::null());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        // Keep env minimal: allowlist only
        cmd.env_clear();
        for k in &self.cfg.env_allowlist {
            if let Ok(v) = std::env::var(k) {
                cmd.env(k, v);
            }
        }

        let mut child = cmd.spawn().context("failed to spawn cargo")?;
        let timeout = Duration::from_millis(self.cfg.timeout_ms);

        let status_opt = child.wait_timeout(timeout).context("wait_timeout failed")?;

        if status_opt.is_none() {
            let _ = child.kill();
            return Ok(ActionResult::error(
                "Timeout",
                format!(
                    "cargo {subcommand} timed out after {}ms",
                    self.cfg.timeout_ms
                ),
            ));
        }

        let output = child
            .wait_with_output()
            .context("wait_with_output failed")?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let code = output.status.code().unwrap_or(-1);

        let ok = output.status.success();
        let kind = format!("Cargo{}", subcommand.to_ascii_uppercase());

        Ok(ActionResult::success(
            kind,
            if ok { "ok" } else { "failed" },
            Some(json!({
                "exit_code": code,
                "stdout": stdout,
                "stderr": stderr
            })),
        ))
    }

    pub fn run_command(&self, program: &str, args: &[String]) -> Result<ActionResult> {
        let mut cmd = Command::new(program);
        for arg in args {
            cmd.arg(arg);
        }

        cmd.current_dir(self.policy.repo_root());
        cmd.stdin(Stdio::null());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let output = cmd.output().context("failed to execute command")?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success() {
            Ok(ActionResult::success("CommandExecuted", stdout, None))
        } else {
            Ok(ActionResult::error("CommandFailed", stderr))
        }
    }
}
