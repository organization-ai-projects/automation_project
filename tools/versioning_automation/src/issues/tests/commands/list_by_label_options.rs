use crate::issues::commands::ListByLabelOptions;

#[test]
fn list_by_label_options_holds_label_and_repo() {
    let options = ListByLabelOptions {
        label: "priority".to_string(),
        repo: Some("owner/repo".to_string()),
    };

    assert_eq!(options.label, "priority");
    assert_eq!(options.repo.as_deref(), Some("owner/repo"));
}
