// projects/products/unstable/autonomous_dev_ai/src/tools/pr_description_generator.rs
use super::{Tool, ToolResult};
use crate::error::{AgentError, AgentResult};
use std::process::Command;

/// PR description generator tool.
pub struct PrDescriptionGenerator;

impl Tool for PrDescriptionGenerator {
    fn name(&self) -> &str {
        "generate_pr_description"
    }

    fn execute(&self, args: &[String]) -> AgentResult<ToolResult> {
        let script_path = "scripts/versioning/file_versioning/github/generate_pr_description.sh";

        let main_pr = args.first().cloned().ok_or_else(|| {
            AgentError::Tool("Missing main PR number for generate_pr_description".to_string())
        })?;
        let output_file = args
            .get(1)
            .cloned()
            .unwrap_or_else(|| "pr_description.md".to_string());

        let output = Command::new(script_path)
            .arg(main_pr)
            .arg(output_file)
            .output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();

                if out.status.success() {
                    Ok(ToolResult {
                        success: true,
                        output: stdout,
                        error: if stderr.is_empty() {
                            None
                        } else {
                            Some(stderr)
                        },
                    })
                } else {
                    Ok(ToolResult {
                        success: false,
                        output: stdout,
                        error: Some(if stderr.is_empty() {
                            format!("generate_pr_description failed with status {}", out.status)
                        } else {
                            stderr
                        }),
                    })
                }
            }
            Err(e) => Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!("failed to execute {}: {}", script_path, e)),
            }),
        }
    }

    fn is_reversible(&self) -> bool {
        false
    }
}
