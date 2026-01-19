// projects/libraries/command_runner/src/string_manipulation.rs
use std::process::ExitStatus;

use crate::{CommandError, CommandInfo};

use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, CommandError>;

/// Converts a slice of string slices to a vector of owned strings.
pub fn args_vec(args: &[&str]) -> Vec<String> {
    args.iter().map(|s| s.to_string()).collect()
}

/// Converts an `ExitStatus` to a human-readable string.
pub fn status_string(status: ExitStatus) -> String {
    match status.code() {
        Some(code) => code.to_string(),
        None => "terminated_by_signal".to_string(),
    }
}

/// Validates that the given program string is not empty.
/// Returns an error if the program is invalid.
pub fn validate_input(program: &str) -> StdResult<(), CommandError> {
    if program.trim().is_empty() {
        return Err(CommandError::InvalidInput {
            info: CommandInfo {
                program: program.to_string(),
                args: vec![],
            },
            reason: "program is empty".to_string(),
        });
    }
    Ok(())
}
