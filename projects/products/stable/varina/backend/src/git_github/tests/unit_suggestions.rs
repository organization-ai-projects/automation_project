// projects/products/varina/backend/src/git_github/tests/unit_suggestions.rs
use crate::autopilot::AutopilotPolicy;
use crate::classified_changes::ClassifiedChanges;
use crate::git_github::policy_suggestions::suggest_policy_from_report;

#[test]
fn test_suggestion_with_unrelated_changes() {
    let report = crate::autopilot::AutopilotReport {
        mode: crate::autopilot::AutopilotMode::DryRun,
        branch: "main".to_string(),
        detached_head: false,
        changes: vec![],
        classified: ClassifiedChanges {
            relevant: vec![],
            unrelated: vec!["unrelated/file.rs".to_string()],
            blocked: vec![],
        },
        plan: crate::autopilot::AutopilotPlan {
            branch: "main".to_string(),
            will_stage: vec![],
            will_commit: false,
            commit_message: String::new(),
            will_push: false,
            notes: vec![],
        },
        applied: false,
        logs: vec![],
    };

    let policy = AutopilotPolicy::default();
    let suggestion = suggest_policy_from_report(&report, &policy);

    // Policy returns None for these fields, only populates notes
    assert_eq!(suggestion.allow_push, None);
    assert_eq!(suggestion.fail_on_unrelated_changes, None);

    // Check that notes contains the expected message about unrelated changes
    assert!(
        suggestion
            .notes
            .iter()
            .any(|note| note.contains("Unrelated changes detected")),
        "Expected notes to contain message about unrelated changes, got: {:?}",
        suggestion.notes
    );
}
