// projects/libraries/command_runner/src/command.rs
use std::fmt;
use std::path::Path;
use std::process::{Command, ExitStatus, Output};
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, CommandError>;

const MAX_LOG_CHARS: usize = 8_000;

fn trim_lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).trim().to_string()
}

/// UTF-8 safe truncation (never panics).
fn truncate_utf8(mut s: String, max_chars_approx: usize) -> String {
    // This uses bytes length as a quick gate; then truncates on char boundary.
    if s.len() <= max_chars_approx {
        return s;
    }
    let mut cut = 0usize;
    for (i, _) in s.char_indices() {
        if i > max_chars_approx {
            break;
        }
        cut = i;
    }
    s.truncate(cut);
    s
}

fn args_vec(args: &[&str]) -> Vec<String> {
    args.iter().map(|s| s.to_string()).collect()
}

fn status_string(status: ExitStatus) -> String {
    match status.code() {
        Some(code) => code.to_string(),
        None => "terminated_by_signal".to_string(),
    }
}

#[derive(Debug, Clone)]
pub enum CommandError {
    InvalidInput {
        program: String,
        reason: String,
    },
    Io {
        program: String,
        args: Vec<String>,
        source: String,
    },
    NonZeroExit {
        program: String,
        args: Vec<String>,
        status: String,
        stdout: String,
        stderr: String,
    },
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::InvalidInput { program, reason } => {
                write!(f, "Invalid command input for '{program}': {reason}")
            }
            CommandError::Io {
                program,
                args,
                source,
            } => write!(f, "Failed to run: {program} {:?}: {source}", args),
            CommandError::NonZeroExit {
                program,
                args,
                status,
                stdout,
                stderr,
            } => write!(
                f,
                "Command failed (exit={status}): {program} {:?}\nstdout: {stdout}\nstderr: {stderr}",
                args
            ),
        }
    }
}

impl std::error::Error for CommandError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureMode {
    /// Return Err(CommandError::NonZeroExit) if exit code != 0.
    Strict,
    /// Return Ok(Output) even if exit code != 0.
    AllowFailure,
}

#[derive(Debug, Clone)]
pub struct CmdLog {
    pub program: String,
    pub args: Vec<String>,
    pub status: String,
    pub stdout: String,
    pub stderr: String,
}

impl CmdLog {
    pub fn push_to(&self, logs: &mut Vec<String>, label: &str) {
        logs.push(format!("[cmd] {label}: {} {:?}", self.program, self.args));
        logs.push(format!("[cmd] {label}: status={}", self.status));
        if !self.stdout.is_empty() {
            logs.push(format!("[cmd] {label}: stdout={}", self.stdout));
        }
        if !self.stderr.is_empty() {
            logs.push(format!("[cmd] {label}: stderr={}", self.stderr));
        }
    }
}

fn validate_input(program: &str) -> Result<()> {
    if program.trim().is_empty() {
        return Err(CommandError::InvalidInput {
            program: program.to_string(),
            reason: "program is empty".to_string(),
        });
    }
    Ok(())
}

/// Canonical runner.
/// - Logs command + status + stdout/stderr (bounded)
/// - Mode controls whether non-zero is an error
pub fn run_cmd_mode(
    repo_path: &Path,
    program: &str,
    args: &[&str],
    mode: FailureMode,
    logs: &mut Vec<String>,
) -> Result<Output> {
    validate_input(program)?;

    let out = Command::new(program)
        .current_dir(repo_path)
        .args(args)
        .output()
        .map_err(|e| CommandError::Io {
            program: program.to_string(),
            args: args_vec(args),
            source: e.to_string(),
        })?;

    let cmdlog = CmdLog {
        program: program.to_string(),
        args: args_vec(args),
        status: status_string(out.status),
        stdout: truncate_utf8(trim_lossy(&out.stdout), MAX_LOG_CHARS),
        stderr: truncate_utf8(trim_lossy(&out.stderr), MAX_LOG_CHARS),
    };

    // Single source of truth for logs
    cmdlog.push_to(logs, match mode {
        FailureMode::Strict => "strict",
        FailureMode::AllowFailure => "allow_failure",
    });

    if mode == FailureMode::Strict && !out.status.success() {
        return Err(CommandError::NonZeroExit {
            program: cmdlog.program,
            args: cmdlog.args,
            status: cmdlog.status,
            stdout: cmdlog.stdout,
            stderr: cmdlog.stderr,
        });
    }

    Ok(out)
}

/// Strict: non-zero exit becomes an error.
pub fn run_cmd_ok(repo_path: &Path, program: &str, args: &[&str], logs: &mut Vec<String>) -> Result<Output> {
    run_cmd_mode(repo_path, program, args, FailureMode::Strict, logs)
}

/// Permissive: always returns Output (caller checks status).
pub fn run_cmd_allow_failure(
    repo_path: &Path,
    program: &str,
    args: &[&str],
    logs: &mut Vec<String>,
) -> Result<Output> {
    run_cmd_mode(repo_path, program, args, FailureMode::AllowFailure, logs)
}
