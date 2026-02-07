// projects/products/varina/backend/src/git_github/tests/unit_suggestions.rs
mod unit_tests {
    use crate::git_github::policy_suggestions::{PolicySuggestion, suggest_policy_from_report};
    use crate::autopilot::AutopilotPolicy;
    use crate::tests::test_helpers::AutopilotReportBuilder;
    use crate::classified_changes::ClassifiedChanges;

    #[test]
    fn test_suggestion_with_unrelated_changes() {
        let report = AutopilotReportBuilder::new()
            .branch("main")
            .classified(ClassifiedChanges {
                relevant: vec![],
                unrelated: vec!["unrelated/file.rs".to_string()],
                blocked: vec![],
            })
            .build();

        let policy = AutopilotPolicy::default();
        let suggestion = suggest_policy_from_report(&report, &policy);

        // Policy returns None for these fields, only populates notes
        assert_eq!(suggestion.allow_push, None);
        assert_eq!(suggestion.fail_on_unrelated_changes, None);
        
        // Check that notes contains the expected message about unrelated changes
        assert!(
            suggestion.notes.iter().any(|note| note.contains("Unrelated changes detected")),
            "Expected notes to contain message about unrelated changes, got: {:?}",
            suggestion.notes
        );
    }
}
