// projects/products/unstable/autonomous_dev_ai/src/tools/test_runner.rs
use super::{Tool, ToolResult, run_with_timeout::run_with_timeout};
use crate::error::{AgentError, AgentResult};
use std::time::Duration;

use super::constants::DEFAULT_TOOL_TIMEOUT_SECS;

/// Test runner tool.
pub struct TestRunner;

impl Tool for TestRunner {
    fn name(&self) -> &str {
        "run_tests"
    }

    fn execute(&self, args: &[String]) -> AgentResult<ToolResult> {
        let (program, cmd_args) = build_test_command(args)?;
        if !is_allowed_test_program(&program) {
            return Err(AgentError::PolicyViolation(format!(
                "test program '{program}' is not allowed"
            )));
        }

        run_with_timeout(
            &program,
            &cmd_args,
            Duration::from_secs(DEFAULT_TOOL_TIMEOUT_SECS),
        )
    }

    fn is_reversible(&self) -> bool {
        true
    }
}

fn is_allowed_test_program(program: &str) -> bool {
    let configured = std::env::var("AUTONOMOUS_ALLOWED_TEST_PROGRAMS")
        .unwrap_or_else(|_| "cargo,pytest".to_string());
    configured
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .any(|allowed| allowed == program)
}

fn build_test_command(args: &[String]) -> AgentResult<(String, Vec<String>)> {
    if let Ok(custom_cmd) = std::env::var("AUTONOMOUS_TEST_COMMAND") {
        let parts: Vec<String> = custom_cmd
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let program = parts.first().cloned().ok_or_else(|| {
            AgentError::Tool("AUTONOMOUS_TEST_COMMAND cannot be empty".to_string())
        })?;
        let mut command_args = parts[1..].to_vec();
        command_args.extend_from_slice(args);
        return Ok((program, command_args));
    }

    if let Some(program) = args.first().cloned() {
        return Ok((program, args[1..].to_vec()));
    }

    Ok((
        "cargo".to_string(),
        vec!["test".to_string(), "--all-targets".to_string()],
    ))
}
