//! tools/versioning_automation/src/category_resolver.rs
use crate::pr::{
    PrIssueContextOptions, issue_category_from_labels, load_issue_context_payload,
    resolve_effective_category,
};

pub(crate) fn resolve_issue_outcome_category(issue_key: &str, default_category: &str) -> String {
    let opts = PrIssueContextOptions {
        issue_number: issue_key.trim_start_matches('#').to_string(),
        repo: None,
    };
    let (labels_raw, title_category, _) = load_issue_context_payload(&opts);
    let label_category = issue_category_from_labels(&labels_raw);
    resolve_effective_category(label_category, &title_category, default_category)
}

pub(crate) fn classify_title(title: &str) -> &'static str {
    let lower = title.to_lowercase();

    if lower.starts_with("merge ") || lower.contains("main into") || lower.contains("dev into") {
        return "Synchronization";
    }
    if lower.starts_with("fix") || lower.contains("bug") || lower.contains("hotfix") {
        return "Bug Fixes";
    }
    if lower.starts_with("refactor")
        || lower.starts_with("chore")
        || lower.contains("cleanup")
        || lower.contains("maintainability")
    {
        return "Refactoring";
    }
    "Features"
}
