// projects/libraries/command_runner/src/string_manipulation.rs
use std::process::ExitStatus;

use crate::{CommandError, CommandInfo};

use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, CommandError>;

/// Converts a byte slice to a trimmed UTF-8 string.
pub fn trim_lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).trim().to_string()
}

/// UTF-8 safe truncation (never panics).
pub fn truncate_utf8(mut s: String, max_chars_approx: usize) -> String {
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
