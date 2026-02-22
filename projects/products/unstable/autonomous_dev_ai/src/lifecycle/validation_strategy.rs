// projects/products/unstable/autonomous_dev_ai/src/lifecycle/validation_strategy.rs
// Context-aware validation command selection for autonomous execution.

#[derive(Debug, Clone)]
pub struct ValidationCommandPlan {
    pub description: String,
    pub command_tokens: Vec<String>,
}

pub fn select_validation_command(
    goal: &str,
    package_name: &str,
    execution_mode: &str,
) -> Option<ValidationCommandPlan> {
    let goal_lc = goal.to_ascii_lowercase();
    if !contains_validation_intent(&goal_lc) {
        return None;
    }

    let allow_test = allows_subcommand(execution_mode, "test");
    let allow_clippy = allows_subcommand(execution_mode, "clippy");
    let allow_check = allows_subcommand(execution_mode, "check");

    if goal_lc.contains("clippy") || goal_lc.contains("lint") {
        if !allow_clippy {
            return fallback_check(package_name, allow_check);
        }
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
        if !allow_test {
            return fallback_check(package_name, allow_check);
        }
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

    fallback_check(package_name, allow_check)
}

fn contains_validation_intent(goal_lc: &str) -> bool {
    [
        "test", "clippy", "lint", "check", "build", "compile", "validate", "ci",
    ]
    .iter()
    .any(|token| goal_lc.contains(token))
}

fn allows_subcommand(execution_mode: &str, subcommand: &str) -> bool {
    match execution_mode {
        "ci_bound" => matches!(subcommand, "check" | "test" | "clippy"),
        "local" | "dev" => matches!(subcommand, "check" | "test" | "clippy" | "build"),
        _ => subcommand == "check",
    }
}

fn fallback_check(package_name: &str, allow_check: bool) -> Option<ValidationCommandPlan> {
    if !allow_check {
        return None;
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
