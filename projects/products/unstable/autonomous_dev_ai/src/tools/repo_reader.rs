//projects/products/unstable/autonomous_dev_ai/src/tools/repo_reader.rs
use super::{Tool, ToolResult};
use crate::error::{AgentError, AgentResult};

/// Repository reader tool.
pub struct RepoReader;

impl Tool for RepoReader {
    fn name(&self) -> &str {
        "read_file"
    }

    fn execute(&self, args: &[String]) -> AgentResult<ToolResult> {
        if args.is_empty() {
            return Err(AgentError::Tool("Missing file path".to_string()));
        }

        let path = &args[0];
        match std::fs::read_to_string(path) {
            Ok(content) => Ok(ToolResult {
                success: true,
                output: content,
                error: None,
            }),
            Err(e) => Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
            }),
        }
    }

    fn is_reversible(&self) -> bool {
        true
    }
}
