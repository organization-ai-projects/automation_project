// projects/libraries/command_runner/src/command_runner.rs
use std::{
    path::Path,
    process::{Command, Output},
};

use crate::{CmdLog, CommandError, CommandInfo, FailureMode, args_vec, validate_input};

/// Strict: non-zero exit becomes an error.
pub fn run_cmd_ok(
    repo_path: &Path,
    program: &str,
    args: &[&str],
    logs: &mut Vec<String>,
) -> Result<Output, CommandError> {
    run_cmd_mode(repo_path, program, args, FailureMode::Strict, logs)
}

/// Permissive: always returns Output (caller checks status).
pub fn run_cmd_allow_failure(
    repo_path: &Path,
    program: &str,
    args: &[&str],
    logs: &mut Vec<String>,
) -> Result<Output, CommandError> {
    run_cmd_mode(repo_path, program, args, FailureMode::AllowFailure, logs)
}

/// Canonical runner.
/// - Logs command + status + stdout/stderr (bounded)
/// - Mode controls whether non-zero is an error
fn run_cmd_mode(
    repo_path: &Path,
    program: &str,
    args: &[&str],
    mode: FailureMode,
    logs: &mut Vec<String>,
) -> Result<Output, CommandError> {
    validate_input(program)?;

    let args_vec = args_vec(args);
    let out = Command::new(program)
        .current_dir(repo_path)
        .args(args)
        .output()
        .map_err(|e| CommandError::Io {
            info: CommandInfo {
                program: program.to_string(),
                args: args_vec.clone(),
            },
            source: e.to_string(),
        })?;

    let cmdlog = CmdLog::new(program, args_vec, &out);

    // Single source of truth for logs
    cmdlog.push_to(
        logs,
        match mode {
            FailureMode::Strict => "strict",
            FailureMode::AllowFailure => "allow_failure",
        },
    );

    if mode == FailureMode::Strict && !out.status.success() {
        return Err(CommandError::NonZeroExit { log: cmdlog });
    }

    Ok(out)
}
