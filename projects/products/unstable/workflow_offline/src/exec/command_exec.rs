use crate::exec::exec_result::ExecResult;
use std::process::Command;

/// Abstraction over shell command execution.
/// All spawned-process side-effects are isolated behind this type.
pub struct CommandExec {
    command: String,
    args: Vec<String>,
}

impl CommandExec {
    pub fn new(command: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            command: command.into(),
            args,
        }
    }

    /// Executes the command and returns the captured result.
    /// Never panics; a failure to spawn is reported as exit code -1.
    pub fn execute(&self) -> ExecResult {
        match Command::new(&self.command).args(&self.args).output() {
            Ok(output) => ExecResult {
                exit_code: output.status.code().unwrap_or(-1),
                stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            },
            Err(e) => ExecResult {
                exit_code: -1,
                stdout: String::new(),
                stderr: format!("failed to spawn `{}`: {e}", self.command),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(unix)]
    fn echo_returns_zero() {
        let exec = CommandExec::new("echo", vec!["hello".to_string()]);
        let result = exec.execute();
        assert_eq!(result.exit_code, 0);
        assert!(result.stdout.contains("hello"));
    }

    #[test]
    fn nonexistent_command_returns_minus_one() {
        let exec =
            CommandExec::new("__nonexistent_cmd_xyz_workflow_offline__", vec![]);
        let result = exec.execute();
        assert_eq!(result.exit_code, -1);
        assert!(!result.stderr.is_empty());
    }
}
