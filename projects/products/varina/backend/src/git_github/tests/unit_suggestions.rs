// projects/products/varina/backend/src/git_github/tests/unit_suggestions.rs
#[cfg(test)]
mod unit_tests {
    use super::super::suggestions::{PolicySuggestion, suggest_policy_from_report};
    use crate::autopilot::{AutopilotPolicy, AutopilotReport};
    use crate::classified_changes::ClassifiedChanges;

    #[test]
    fn test_suggestion_with_unrelated_changes() {
        let report = AutopilotReport {
            mode: Default::default(),
            branch: "main".to_string(),
            detached_head: false,
            changes: vec!["README.md".to_string()],
            classified: ClassifiedChanges {
                relevant: vec![],
                unrelated: vec!["unrelated/file.rs".to_string()],
                blocked: vec![],
            },
            plan: Default::default(),
            applied: false,
            logs: vec![],
        };

        let policy = AutopilotPolicy::default();
        let suggestion = suggest_policy_from_report(&report, &policy);

        assert_eq!(suggestion.allow_push, None);
        assert_eq!(suggestion.fail_on_unrelated_changes, Some(false));
        assert!(
            suggestion
                .notes
                .contains(&"Unrelated changes detected.".to_string())
        );
    }
}
