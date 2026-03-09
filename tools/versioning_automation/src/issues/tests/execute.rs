use crate::issues;

#[test]
fn execute_create_dry_run_still_works_after_refactor() {
    let args = vec![
        "create".to_string(),
        "--title".to_string(),
        "feat(example): dry run".to_string(),
        "--context".to_string(),
        "ctx".to_string(),
        "--problem".to_string(),
        "problem".to_string(),
        "--acceptance".to_string(),
        "criterion".to_string(),
        "--dry-run".to_string(),
    ];
    let code = issues::run(&args);
    assert_eq!(code, 0);
}
