use crate::pr::commands::pr_duplicate_actions_options::PrDuplicateActionsOptions;
use crate::pr::duplicate_actions::run_duplicate_actions;

#[test]
fn duplicate_actions_rejects_invalid_mode() {
    let code = run_duplicate_actions(PrDuplicateActionsOptions {
        text: "#1|#2".to_string(),
        mode: "invalid".to_string(),
        repo: "organization/repository".to_string(),
        assume_yes: false,
    });
    assert_eq!(code, 2);
}

#[test]
fn duplicate_actions_rejects_missing_repo() {
    let code = run_duplicate_actions(PrDuplicateActionsOptions {
        text: "#1|#2".to_string(),
        mode: "safe".to_string(),
        repo: "".to_string(),
        assume_yes: false,
    });
    assert_eq!(code, 2);
}
