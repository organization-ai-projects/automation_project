// projects/products/unstable/autonomous_dev_ai/src/tools/mod.rs

//! Tool system - all tools are symbolic wrappers

use crate::error::{AgentError, AgentResult};
use serde::{Deserialize, Serialize};

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
