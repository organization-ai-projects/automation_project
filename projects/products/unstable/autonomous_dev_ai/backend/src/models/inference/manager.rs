//! projects/products/unstable/autonomous_dev_ai/src/models/inference/manager.rs
use crate::{memory::FailureEntry, models::DecisionEntry};

pub(crate) fn infer_failure_kind(entry: &FailureEntry) -> String {
    let text = format!(
        "{} {}",
        entry.description.to_ascii_lowercase(),
        entry.error.to_ascii_lowercase()
    );

    if text.contains("policy") || text.contains("authorization") {
        return "policy".to_string();
    }
    if text.contains("timeout") {
        return "timeout".to_string();
    }
    if text.contains("circuit") {
        return "circuit_breaker".to_string();
    }
    if text.contains("resource") || text.contains("budget") {
        return "resource".to_string();
    }
    if text.contains("test") || text.contains("validation") {
        return "validation".to_string();
    }
    if text.contains("tool") {
        return "tool".to_string();
    }
    "other".to_string()
}

pub(crate) fn infer_failure_tool(entry: &FailureEntry) -> String {
    let text = format!(
        "{} {}",
        entry.description.to_ascii_lowercase(),
        entry.error.to_ascii_lowercase()
    );
    for tool in [
        "run_tests",
        "read_file",
        "git_commit",
        "generate_pr_description",
        "apply_patch",
        "format_code",
        "git_branch",
        "create_pr",
    ] {
        if text.contains(tool) {
            return tool.to_string();
        }
    }
    "unknown".to_string()
}

pub(crate) fn infer_decision_action(entry: &DecisionEntry) -> String {
    let text = format!(
        "{} {}",
        entry.description.to_ascii_lowercase(),
        entry.symbolic_decision.as_str().to_ascii_lowercase()
    );
    for action in [
        "run_tests",
        "read_file",
        "git_commit",
        "generate_pr_description",
        "apply_patch",
        "format_code",
        "git_branch",
        "create_pr",
    ] {
        if text.contains(action) {
            return action.to_string();
        }
    }
    "other".to_string()
}
