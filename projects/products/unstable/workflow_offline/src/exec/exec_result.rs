use serde::{Deserialize, Serialize};

/// The result of executing a single shell command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecResult {
    /// Exit code returned by the process (0 = success).
    pub exit_code: i32,
    /// Standard output captured from the process.
    pub stdout: String,
    /// Standard error captured from the process.
    pub stderr: String,
}

impl ExecResult {
    /// Returns `true` if the command exited successfully (exit code 0).
    pub fn success(&self) -> bool {
        self.exit_code == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_on_zero_exit() {
        let r = ExecResult {
            exit_code: 0,
            stdout: String::new(),
            stderr: String::new(),
        };
        assert!(r.success());
    }

    #[test]
    fn failure_on_nonzero_exit() {
        let r = ExecResult {
            exit_code: 1,
            stdout: String::new(),
            stderr: String::new(),
        };
        assert!(!r.success());
    }
}
