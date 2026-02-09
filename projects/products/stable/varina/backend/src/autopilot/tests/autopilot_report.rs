// projects/products/stable/varina/backend/src/autopilot/tests/autopilot_report.rs

use crate::autopilot::{AutopilotMode, AutopilotPlan, AutopilotReport};
use crate::classified_changes::ClassifiedChanges;

#[test]
fn test_autopilot_report_usage() {
    let plan = AutopilotPlan {
        branch: "main".to_string(),
        will_stage: vec!["file1.rs".to_string()],
        will_commit: true,
        commit_message: "Initial commit".to_string(),
        will_push: true,
        notes: vec!["Note 1".to_string()],
    };

    let report = AutopilotReport {
        mode: AutopilotMode::DryRun,
        branch: "main".to_string(),
        detached_head: false,
        changes: vec![],
        classified: ClassifiedChanges {
            blocked: vec!["blocked_file.rs".to_string()],
            relevant: vec!["relevant_file.rs".to_string()],
            unrelated: vec!["unrelated_file.rs".to_string()],
        },
        plan,
        applied: true,
        logs: vec!["Log entry".to_string()],
    };

    assert_eq!(report.mode, AutopilotMode::DryRun);
    assert_eq!(report.branch, "main");
    assert!(!report.detached_head);
    assert!(report.changes.is_empty());
    assert!(report.applied);
    assert_eq!(report.logs.len(), 1);

    // Using the classified and plan fields
    assert_eq!(report.classified.blocked[0], "blocked_file.rs");
    assert_eq!(report.classified.relevant[0], "relevant_file.rs");
    assert_eq!(report.classified.unrelated[0], "unrelated_file.rs");

    assert_eq!(report.plan.branch, "main");
    assert!(report.plan.will_commit);
    assert_eq!(report.plan.commit_message, "Initial commit");
    assert!(report.plan.will_push);
    assert_eq!(report.plan.notes.len(), 1);
}
