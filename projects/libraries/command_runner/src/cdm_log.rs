// projects/libraries/command_runner/src/cdm_log.rs
use std::process::Output;

use common::{trim_lossy, truncate_utf8};

use crate::{CommandInfo, MAX_LOG_CHARS, status_string};

#[derive(Debug, Clone)]
pub struct CmdLog {
    pub info: CommandInfo,
    pub status: String,
    pub stdout: String,
    pub stderr: String,
}

impl CmdLog {
    pub fn new(program: &str, args: Vec<String>, out: &Output) -> Self {
        Self {
            info: CommandInfo {
                program: program.to_string(),
                args,
            },
            status: status_string(out.status),
            stdout: truncate_utf8(trim_lossy(&out.stdout), MAX_LOG_CHARS),
            stderr: truncate_utf8(trim_lossy(&out.stderr), MAX_LOG_CHARS),
        }
    }

    pub fn push_to(&self, logs: &mut Vec<String>, label: &str) {
        logs.push(format!(
            "[cmd] {label}: {} {:?}",
            self.info.program, self.info.args
        ));
        logs.push(format!("[cmd] {label}: status={}", self.status));
        if !self.stdout.is_empty() {
            logs.push(format!("[cmd] {label}: stdout={}", self.stdout));
        }
        if !self.stderr.is_empty() {
            logs.push(format!("[cmd] {label}: stderr={}", self.stderr));
        }
    }
}
