use crate::issues::commands::HasLabelOptions;

#[test]
fn has_label_options_can_be_built() {
    let value = HasLabelOptions {
        issue: "42".to_string(),
        label: "done".to_string(),
        repo: Some("owner/repo".to_string()),
    };
    assert_eq!(value.issue, "42");
    assert_eq!(value.label, "done");
    assert_eq!(value.repo.as_deref(), Some("owner/repo"));
}
