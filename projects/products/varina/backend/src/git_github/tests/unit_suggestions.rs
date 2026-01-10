// projects/products/varina/backend/src/git_github/tests/unit_suggestions.rs
#[cfg(test)]
mod unit_tests {
    use super::super::suggestions::{PolicySuggestion, suggest_policy_from_report};
    use crate::autopilot::{AutopilotPolicy, AutopilotReport, ClassifiedChanges, GitChange};

    #[test]
    fn test_suggestion_with_unrelated_changes() {
        let report = AutopilotReport {
            mode: Default::default(),
            branch: "main".to_string(),
            detached_head: false,
            changes: vec![GitChange {
                xy: [32, 77],
                path: "docs/versioning/git-github.md".to_string(),
                orig_path: None,
            }],
            classified: ClassifiedChanges {
                relevant: vec![],
                unrelated: vec![GitChange {
                    xy: [32, 77],
                    path: "unrelated/file.rs".to_string(),
                    orig_path: None,
                }],
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
