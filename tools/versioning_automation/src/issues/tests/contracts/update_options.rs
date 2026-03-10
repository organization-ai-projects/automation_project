use crate::issues::contracts::cli::UpdateOptions;

#[test]
fn update_options_can_be_built() {
    let value = UpdateOptions {
        issue: "42".to_string(),
        repo: None,
        edit_args: vec![("--title".to_string(), "new".to_string())],
    };
    assert_eq!(value.issue, "42");
}
