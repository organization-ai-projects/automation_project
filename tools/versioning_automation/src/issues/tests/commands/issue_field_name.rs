use crate::issues::commands::IssueFieldName;

#[test]
fn issue_field_name_title_variant_exists() {
    let value = IssueFieldName::Title;
    assert_eq!(value, IssueFieldName::Title);
}
