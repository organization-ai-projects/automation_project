// projects/products/unstable/autonomous_dev_ai/src/symbolic/policy.rs

use serde::{Deserialize, Serialize};

/// Policy engine for validating actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEngine {
    pub allowed_tools: Vec<String>,
    pub forbidden_patterns: Vec<String>,
}

impl PolicyEngine {
    pub fn new() -> Self {
        Self {
            allowed_tools: vec![
                "read_file".to_string(),
                "search_code".to_string(),
                "apply_patch".to_string(),
                "run_tests".to_string(),
                "format_code".to_string(),
                "git_commit".to_string(),
                "git_branch".to_string(),
                "create_pr".to_string(),
                "generate_pr_description".to_string(),
            ],
            forbidden_patterns: vec![
                "force-push".to_string(),
                "rm -rf".to_string(),
                "/etc/".to_string(),
                "sudo ".to_string(),
            ],
        }
    }

    pub fn validate_action(&self, action: &str) -> bool {
        // Check for forbidden patterns
        for pattern in &self.forbidden_patterns {
            if action.contains(pattern) {
                return false;
            }
        }
        true
    }

    pub fn is_tool_allowed(&self, tool: &str) -> bool {
        self.allowed_tools.contains(&tool.to_string())
    }
}

impl Default for PolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}
