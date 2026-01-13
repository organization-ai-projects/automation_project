// projects/products/code_agent_sandbox/src/command_runner.rs
// NOTE: validate_args returns Rust errors (anyhow::Error), NOT ActionResult.
// ActionResult is protocol output, never used as an error type.

use anyhow::{Result, bail};
use command_runner::{run_cmd_allow_failure, run_cmd_ok};
use protocol::json;
use std::path::Path;

use crate::{actions::ActionResult, policies::Policy, runner_config::RunnerConfig};

#[derive(Clone)]
pub struct CommandRunner {
    pub policy: Policy,
    pub cfg: RunnerConfig,
}

impl CommandRunner {
    pub fn new(policy: Policy, cfg: RunnerConfig) -> Self {
        Self { policy, cfg }
    }

    fn validate_args(args: &[String], disallowed_patterns: &[&str]) -> Result<()> {
        for a in args {
            if disallowed_patterns.iter().any(|p| a.contains(p)) {
                bail!("suspicious arg: {a}");
            }
            let path = Path::new(a);
            if path.is_absolute() || a.starts_with("C:\\") {
                bail!("absolute path not allowed: {a}");
            }
        }
        Ok(())
    }

    pub fn run_cargo(&self, subcommand: &str, args: &[String]) -> Result<ActionResult> {
        // Validate subcommand first (policy decision => ActionResult, not crash)
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

        // Validate args (policy decision => ActionResult)
        if let Err(e) = Self::validate_args(
            args,
            &[
                "..",
                "\\0",
                "--manifest-path",
                "--target-dir",
                "--config",
                "-Z",
                "--release",
            ],
        ) {
            return Ok(ActionResult::error("PolicyViolation", e.to_string()));
        }

        let mut logs = Vec::new();
        let output = run_cmd_ok(
            self.policy.work_root(),
            &self.cfg.cargo_path,
            [subcommand, "--locked", "--offline", "--frozen"]
                .iter()
                .copied()
                .chain(args.iter().map(|s| s.as_str()))
                .collect::<Vec<_>>()
                .as_slice(),
            &mut logs,
        )?;

        Ok(ActionResult::success(
            "CommandExecuted",
            "ok",
            Some(json!({
                "exitCode": output.status.code().unwrap_or(-1),
                "stdout": truncate(&String::from_utf8_lossy(&output.stdout), 2000),
                "stderr": truncate(&String::from_utf8_lossy(&output.stderr), 2000),
            })),
        ))
    }

    pub fn run_in_bunker(&self, program: &str, args: &[String]) -> Result<ActionResult> {
        if let Err(e) = Self::validate_args(
            args,
            &["..", "\\0", "--dangerous", "-rf", "--no-preserve-root"],
        ) {
            return Ok(ActionResult::error("PolicyViolation", e.to_string()));
        }

        let mut logs = Vec::new();
        let output = match run_cmd_allow_failure(
            self.policy.work_root(),
            program,
            args.iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .as_slice(),
            &mut logs,
        ) {
            Ok(output) => output,
            Err(e) => {
                return Ok(ActionResult::error(
                    "CommandExecutionFailed",
                    format!("Failed to execute command: {e:?}"),
                ));
            }
        };

        Ok(ActionResult::success(
            "CommandExecuted",
            "ok",
            Some(json!({
                "exitCode": output.status.code().unwrap_or(-1),
                "stdout": truncate(&String::from_utf8_lossy(&output.stdout), 2000),
                "stderr": truncate(&String::from_utf8_lossy(&output.stderr), 2000),
            })),
        ))
    }

    pub fn requires_bunker(subcommand: &str) -> bool {
        const BUNKER_COMMANDS: [&str; 2] = ["install", "publish"];
        BUNKER_COMMANDS.contains(&subcommand)
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
