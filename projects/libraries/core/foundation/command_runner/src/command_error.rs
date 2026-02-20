// projects/libraries/command_runner/src/command_error.rs
use crate::{CmdLog, CommandInfo};
use std::fmt;

#[derive(Debug, Clone)]
pub enum CommandError {
    InvalidInput { info: CommandInfo, reason: String },
    Io { info: CommandInfo, source: String },
    NonZeroExit { log: CmdLog },
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::InvalidInput { info, reason } => {
                write!(
                    f,
                    "Invalid command input for '{}': {}",
                    info.program, reason
                )
            }
            CommandError::Io { info, source } => write!(
                f,
                "Failed to run: {} {:?}: {}",
                info.program, info.args, source
            ),
            CommandError::NonZeroExit { log } => write!(
                f,
                "Command failed (exit={}): {} {:?}\nstdout: {}\nstderr: {}",
                log.status, log.info.program, log.info.args, log.stdout, log.stderr
            ),
        }
    }
}

impl std::error::Error for CommandError {}
