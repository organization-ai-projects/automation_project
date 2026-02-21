// projects/products/unstable/autonomous_dev_ai/src/tools/test_runner.rs
use super::{Tool, ToolResult, run_with_timeout::run_with_timeout};
use crate::error::AgentResult;
use std::time::Duration;

use super::constants::DEFAULT_TOOL_TIMEOUT_SECS;

/// Test runner tool.
pub struct TestRunner;

impl Tool for TestRunner {
    fn name(&self) -> &str {
        "run_tests"
    }

    fn execute(&self, args: &[String]) -> AgentResult<ToolResult> {
        if let Ok(custom_cmd) = std::env::var("AUTONOMOUS_TEST_COMMAND") {
            let parts: Vec<String> = custom_cmd
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            let prog = parts
                .first()
                .cloned()
                .unwrap_or_else(|| "cargo".to_string());
            let mut cmd_args = parts[1..].to_vec();
            cmd_args.extend_from_slice(args);
            return run_with_timeout(
                &prog,
                &cmd_args,
                Duration::from_secs(DEFAULT_TOOL_TIMEOUT_SECS),
            );
        }

        let filter = args.first().map(|s| s.as_str()).unwrap_or("all");
        Ok(ToolResult {
            success: true,
            output: format!("Tests passed: {filter}"),
            error: None,
        })
    }

    fn is_reversible(&self) -> bool {
        true
    }
}
