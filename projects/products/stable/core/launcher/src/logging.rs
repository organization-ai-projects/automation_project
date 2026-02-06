// projects/products/stable/core/launcher/src/logging.rs
use std::io::{self, Write};

/// Centralized logging function to handle both stdout and stderr.
///
/// # Arguments
/// * `message` - The message to log.
/// * `is_error` - Whether the message is an error (stderr) or not (stdout).
/// * `context` - Additional context to include in the log.
pub(crate) fn log_message(message: &str, is_error: bool, context: &str) {
    if is_error {
        let _ = writeln!(io::stderr(), "[{}] {}", context, message);
    } else {
        let _ = writeln!(io::stdout(), "[{}] {}", context, message);
    }
}
