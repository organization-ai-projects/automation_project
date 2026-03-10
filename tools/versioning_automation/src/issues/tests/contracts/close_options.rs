use crate::issues::contracts::cli::CloseOptions;

#[test]
fn close_options_can_be_built() {
    let value = CloseOptions {
        issue: "42".to_string(),
        repo: None,
        reason: "completed".to_string(),
    };
    assert_eq!(value.reason, "completed");
}
