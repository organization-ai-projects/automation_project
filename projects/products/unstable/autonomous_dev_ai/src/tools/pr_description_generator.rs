// projects/products/unstable/autonomous_dev_ai/src/tools/pr_description_generator.rs
use super::constants::DEFAULT_TOOL_TIMEOUT_SECS;
use super::run_with_timeout::run_with_timeout;
use super::{Tool, ToolResult};
use crate::error::{AgentError, AgentResult};
use std::time::Duration;

/// PR description generator tool.
pub struct PrDescriptionGenerator;

impl Tool for PrDescriptionGenerator {
    fn name(&self) -> &str {
        "generate_pr_description"
    }

    fn execute(&self, args: &[String]) -> AgentResult<ToolResult> {
        let command = "versioning_automation";

        let main_pr = args.first().cloned().ok_or_else(|| {
            AgentError::Tool("Missing main PR number for generate_pr_description".to_string())
        })?;
        let output_file = args
            .get(1)
            .cloned()
            .unwrap_or_else(|| "pr_description.md".to_string());

        let command_args = vec![
            "pr".to_string(),
            "generate-description".to_string(),
            main_pr,
            output_file,
        ];

        run_with_timeout(
            command,
            &command_args,
            Duration::from_secs(DEFAULT_TOOL_TIMEOUT_SECS),
        )
    }

    fn is_reversible(&self) -> bool {
        false
    }
}
