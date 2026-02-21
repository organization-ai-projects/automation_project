use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::memory::{DecisionEntry, FailureEntry};

pub(crate) fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

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

pub(crate) fn top_entry_key(map: &HashMap<String, usize>) -> Option<String> {
    map.iter()
        .max_by_key(|(_, v)| *v)
        .map(|(k, v)| format!("{k}:{v}"))
}
