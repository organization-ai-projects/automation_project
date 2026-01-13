// projects/products/code_agent_sandbox/src/command_runner.rs
use std::{
    path::Path,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use anyhow::{Context, Result};
use protocol::json;
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

    /// Utility to drain and collect output from a reader.
    fn drain(mut r: impl std::io::Read + Send + 'static) -> thread::JoinHandle<Vec<u8>> {
        thread::spawn(move || {
            let mut buf = Vec::new();
            let _ = r.read_to_end(&mut buf);
            buf
        })
    }

    fn drain_output(handle: Option<thread::JoinHandle<Vec<u8>>>) -> String {
        handle
            .map(|h| String::from_utf8_lossy(&h.join().unwrap_or_default()).to_string())
            .unwrap_or_default()
    }

    /// Centralized argument validation to avoid duplication.
    fn validate_args(args: &[String], disallowed_patterns: &[&str]) -> Result<(), ActionResult> {
        for a in args {
            if disallowed_patterns.iter().any(|p| a.contains(p)) {
                return Err(ActionResult::error(
                    "PolicyViolation",
                    format!("suspicious arg: {a}"),
                ));
            }

            // Check for absolute paths
            let path = Path::new(a);
            if path.is_absolute() || a.starts_with("C:\\") {
                return Err(ActionResult::error(
                    "PolicyViolation",
                    format!("absolute path not allowed: {a}"),
                ));
            }
        }
        Ok(())
    }

    pub fn run_cargo(&self, subcommand: &str, args: &[String]) -> Result<ActionResult> {
        // Valider les arguments spécifiques à cargo
        Self::validate_args(
            args,
            &[
                "..",
                "\\0",
                "--manifest-path",
                "--target-dir",
                "--config",
                "-Z",
                "--release", // Restrict release builds
            ],
        )
        .map_err(|e| anyhow::Error::msg(format!("Validation failed: {:?}", e)))?;

        // Vérifier les sous-commandes autorisées
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

        // Construire et exécuter la commande via run_command
        let mut full_args = vec![
            subcommand.to_string(),
            "--locked".to_string(),
            "--offline".to_string(),
            "--frozen".to_string(),
        ];
        full_args.extend_from_slice(args);
        self.run_command(&self.cfg.cargo_path, &full_args)
    }

    /// Méthode générique pour exécuter des commandes, rendue privée pour usage interne.
    fn run_command(&self, program: &str, args: &[String]) -> Result<ActionResult> {
        // Validate binary
        if !self.cfg.allowed_bins.iter().any(|b| b == program) {
            return Ok(ActionResult::error(
                "PolicyViolation",
                format!("binary not allowed: {program}"),
            ));
        }

        // Validate arguments
        Self::validate_args(args, &["..", "\0"])
            .map_err(|e| anyhow::Error::msg(format!("Validation failed: {:?}", e)))?;

        // Build the command
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

        tracing::info!("Running command: {} {:?}", program, args);
        let mut child = cmd.spawn().context("failed to spawn command")?;

        // Start draining in background threads
        let stdout_h = child.stdout.take().map(Self::drain);
        let stderr_h = child.stderr.take().map(Self::drain);

        let timeout = Duration::from_millis(self.cfg.timeout_ms.max(1));
        let status_opt = child.wait_timeout(timeout).context("wait_timeout failed")?;

        let timed_out = status_opt.is_none();
        if timed_out {
            let _ = child.kill();
            let _ = child.wait();
        }

        // Now collect outputs (won't block forever because process is done/killed)
        let stdout = Self::drain_output(stdout_h);
        let stderr = Self::drain_output(stderr_h);

        match status_opt {
            Some(status) if status.success() => Ok(ActionResult::success(
                "CommandExecuted",
                "ok",
                Some(json!({
                    "exit_code": status.code().unwrap_or(-1),
                    "stdout": truncate(&stdout, 2000),
                    "stderr": truncate(&stderr, 2000)
                })),
            )),
            Some(status) => Ok(ActionResult::error(
                "CommandFailed",
                format!(
                    "exit_code={} stderr={}",
                    status.code().unwrap_or(-1),
                    truncate(&stderr, 2000)
                ),
            )),
            None => Ok(ActionResult::error(
                "Timeout",
                format!("command timed out after {}ms", self.cfg.timeout_ms),
            )),
        }
    }

    /// Exécute une commande dans un environnement isolé (niveau bunker).
    pub fn run_in_bunker(&self, program: &str, args: &[String]) -> Result<ActionResult> {
        // Valider les arguments pour détecter des motifs dangereux
        Self::validate_args(
            args,
            &["..", "\\0", "--dangerous", "-rf", "--no-preserve-root"],
        )
        .map_err(|e| anyhow::Error::msg(format!("Validation failed: {:?}", e)))?;

        // Exécuter la commande via run_command
        self.run_command(program, args)
    }

    /// Determines if a subcommand requires execution in a bunker environment.
    pub fn requires_bunker(subcommand: &str) -> bool {
        // Exemple de logique : certaines sous-commandes nécessitent un bunker
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
