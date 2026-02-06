//! projects/products/varina/backend/src/git_github/policy_suggestions.rs
use crate::autopilot::{AutopilotPolicy, AutopilotReport};

pub struct PolicySuggestion {
    pub allow_push: Option<bool>,
    pub fail_on_unrelated_changes: Option<bool>,
    pub notes: Vec<String>,
}

pub fn suggest_policy_from_report(
    report: &AutopilotReport,
    policy: &AutopilotPolicy,
) -> PolicySuggestion {
    let mut notes = Vec::new();

    // 1) blocked => we NEVER try to "force through"
    if !report.classified.blocked.is_empty() {
        notes.push(
            "Blocked changes detected: remove/relocate these files (or adjust blocked_prefixes if it's a false positive)."
                .to_string(),
        );
    }

    // 2) unrelated => if the policy refuses, we suggest a human action, not an automatic workaround
    if policy.fail_on_unrelated_changes && !report.classified.unrelated.is_empty() {
        notes.push(
            "Unrelated changes detected: separate commit recommended (autopilot refuses by default to avoid magical splits)."
                .to_string(),
        );
    }

    // 3) push => we do not suggest enabling push automatically
    if report.plan.will_push && !policy.allow_push {
        notes.push(
            "The plan includes a push but the policy forbids it (allow_push=false).".to_string(),
        );
    }

    PolicySuggestion {
        allow_push: None,
        fail_on_unrelated_changes: None,
        notes,
    }
}
