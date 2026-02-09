// projects/products/stable/varina/backend/src/autopilot/tests/autopilot_plan.rs

use crate::autopilot::AutopilotPlan;

#[test]
fn test_autopilot_plan_usage() {
    let plan = AutopilotPlan {
        branch: "main".to_string(),
        will_stage: vec!["file1.rs".to_string()],
        will_commit: true,
        commit_message: "Initial commit".to_string(),
        will_push: true,
        notes: vec!["Note 1".to_string()],
    };

    assert_eq!(plan.branch, "main");
    assert!(plan.will_commit);
    assert_eq!(plan.commit_message, "Initial commit");
    assert!(plan.will_push);
    assert_eq!(plan.notes.len(), 1);
}

#[test]
fn test_autopilot_plan_validation() {
    let mut plan = AutopilotPlan {
        branch: String::new(),
        will_stage: vec![],
        will_commit: true,
        commit_message: String::new(),
        will_push: true,
        notes: vec![],
    };

    assert_eq!(
        plan.validate().err(),
        Some("Branch name cannot be empty".to_string())
    );

    plan.branch = "main".to_string();
    assert_eq!(
        plan.validate().err(),
        Some("Commit message cannot be empty".to_string())
    );

    plan.commit_message = "Initial commit".to_string();
    assert!(plan.validate().is_ok());
}
