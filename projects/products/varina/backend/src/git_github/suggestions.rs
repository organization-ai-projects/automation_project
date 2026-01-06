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
    let allow_push = if report.plan.will_commit {
        Some(true)
    } else {
        notes.push("No commits detected; push not recommended.".to_string());
        None
    };

    let fail_on_unrelated_changes = if !report.classified.unrelated.is_empty() {
        notes.push("Unrelated changes detected.".to_string());
        Some(false)
    } else {
        None
    };

    let _ = policy; // Supprimer l'avertissement de variable inutilisée.

    PolicySuggestion {
        allow_push,
        fail_on_unrelated_changes,
        notes,
    }
}
