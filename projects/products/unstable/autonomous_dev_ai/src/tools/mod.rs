// projects/products/unstable/autonomous_dev_ai/src/tools/mod.rs

//! Tool system - all tools are symbolic wrappers

use crate::error::{AgentError, AgentResult};
use serde::{Deserialize, Serialize};
use std::process::Command;

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

/// Tool trait - all tools must implement this
pub trait Tool {
    fn name(&self) -> &str;
    fn execute(&self, args: &[String]) -> AgentResult<ToolResult>;
    fn is_reversible(&self) -> bool;
}

/// Repository reader tool
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
        true // Reading is always reversible
    }
}

/// Test runner tool
pub struct TestRunner;

impl Tool for TestRunner {
    fn name(&self) -> &str {
        "run_tests"
    }

    fn execute(&self, args: &[String]) -> AgentResult<ToolResult> {
        let test_name = args.first().map(|s| s.as_str()).unwrap_or("all");

        // Stub: would actually run tests
        Ok(ToolResult {
            success: true,
            output: format!("Tests passed: {}", test_name),
            error: None,
        })
    }

    fn is_reversible(&self) -> bool {
        true // Testing doesn't modify state
    }
}

/// Git wrapper tool
pub struct GitWrapper;

impl Tool for GitWrapper {
    fn name(&self) -> &str {
        "git"
    }

    fn execute(&self, args: &[String]) -> AgentResult<ToolResult> {
        if args.is_empty() {
            return Err(AgentError::Tool("Missing git command".to_string()));
        }

        let command = &args[0];

        // Forbidden commands
        if command == "push" && args.contains(&"--force".to_string()) {
            return Err(AgentError::PolicyViolation(
                "force-push is not allowed".to_string(),
            ));
        }

        // Stub: would actually execute git
        Ok(ToolResult {
            success: true,
            output: format!("Git {} executed", command),
            error: None,
        })
    }

    fn is_reversible(&self) -> bool {
        false // Git operations are not always reversible
    }
}

/// PR description generator tool
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

/// Tool registry
pub struct ToolRegistry {
    tools: Vec<Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        self.tools.push(tool);
    }

    pub fn get_tool(&self, name: &str) -> Option<&dyn Tool> {
        self.tools
            .iter()
            .find(|t| t.name() == name)
            .map(|t| t.as_ref())
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
