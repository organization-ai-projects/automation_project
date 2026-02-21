//projects/products/unstable/autonomous_dev_ai/src/tools/git_wrapper.rs
use super::constants::{DEFAULT_TOOL_TIMEOUT_SECS, GIT_ALLOWED_SUBCOMMANDS};
use super::run_with_timeout::run_with_timeout;
use super::{Tool, ToolResult};
use crate::error::{AgentError, AgentResult};
use crate::symbolic::{FORCE_PUSH_FORBIDDEN, is_force_push_action};
use std::time::Duration;

/// Git wrapper tool.
pub struct GitWrapper;

impl Tool for GitWrapper {
    fn name(&self) -> &str {
        "git_commit"
    }

    fn execute(&self, args: &[String]) -> AgentResult<ToolResult> {
        if args.is_empty() {
            return Err(AgentError::Tool("Missing git command".to_string()));
        }

        let command = &args[0];

        if is_force_push_action(&args.join(" ")) {
            return Err(AgentError::PolicyViolation(format!(
                "{FORCE_PUSH_FORBIDDEN} is not allowed"
            )));
        }

        if !GIT_ALLOWED_SUBCOMMANDS.contains(&command.as_str()) {
            return Err(AgentError::PolicyViolation(format!(
                "git subcommand '{command}' is not on the allowed list"
            )));
        }

        run_with_timeout("git", args, Duration::from_secs(DEFAULT_TOOL_TIMEOUT_SECS))
    }

    fn is_reversible(&self) -> bool {
        false
    }
}
