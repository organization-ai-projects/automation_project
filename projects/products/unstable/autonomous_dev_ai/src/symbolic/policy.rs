// projects/products/unstable/autonomous_dev_ai/src/symbolic/policy.rs

use serde::{Deserialize, Serialize};

pub const FORCE_PUSH_FORBIDDEN: &str = "force-push";

pub fn is_force_push_action(action: &str) -> bool {
    let lower = action.to_ascii_lowercase();
    if lower.contains("force-push") || lower.contains("force_push") {
        return true;
    }

    let mut has_push = false;
    let mut has_force_flag = false;
    for token in lower.split_whitespace() {
        if token == "push" {
            has_push = true;
        }
        if token == "--force" || token == "-f" {
            has_force_flag = true;
        }
    }

    has_push && has_force_flag
}

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
                "rm -rf".to_string(),
                "/etc/".to_string(),
                "sudo ".to_string(),
            ],
        }
    }

    pub fn validate_action(&self, action: &str) -> bool {
        if is_force_push_action(action) {
            return false;
        }

        let action_lc = action.to_ascii_lowercase();

        // Check for forbidden patterns
        for pattern in &self.forbidden_patterns {
            if action_lc.contains(&pattern.to_ascii_lowercase()) {
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
