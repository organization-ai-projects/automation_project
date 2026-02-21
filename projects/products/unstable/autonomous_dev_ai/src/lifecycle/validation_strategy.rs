// projects/products/unstable/autonomous_dev_ai/src/lifecycle/validation_strategy.rs
// Context-aware validation command selection for autonomous execution.

#[derive(Debug, Clone)]
pub struct ValidationCommandPlan {
    pub description: String,
    pub command_tokens: Vec<String>,
}

pub fn select_validation_command(goal: &str, package_name: &str) -> Option<ValidationCommandPlan> {
    let goal_lc = goal.to_ascii_lowercase();
    if !contains_validation_intent(&goal_lc) {
        return None;
    }

    if goal_lc.contains("clippy") || goal_lc.contains("lint") {
        return Some(ValidationCommandPlan {
            description: "Run clippy on the runtime binary target".to_string(),
            command_tokens: vec![
                "cargo".to_string(),
                "clippy".to_string(),
                "-p".to_string(),
                package_name.to_string(),
                "--bin".to_string(),
                package_name.to_string(),
            ],
        });
    }

    if goal_lc.contains("test") {
        return Some(ValidationCommandPlan {
            description: "Run tests for the runtime binary target".to_string(),
            command_tokens: vec![
                "cargo".to_string(),
                "test".to_string(),
                "-p".to_string(),
                package_name.to_string(),
                "--bin".to_string(),
                package_name.to_string(),
            ],
        });
    }

    Some(ValidationCommandPlan {
        description: "Run check for the runtime binary target".to_string(),
        command_tokens: vec![
            "cargo".to_string(),
            "check".to_string(),
            "-p".to_string(),
            package_name.to_string(),
            "--bin".to_string(),
            package_name.to_string(),
        ],
    })
}

fn contains_validation_intent(goal_lc: &str) -> bool {
    [
        "test", "clippy", "lint", "check", "build", "compile", "validate", "ci",
    ]
    .iter()
    .any(|token| goal_lc.contains(token))
}
