#[test]
fn extract_parent_field_normalizes_parent_value() {
    let body = "Some body\nParent: (EPIC)\nOther: value";
    let parent = crate::parent_field::extract_parent_field(body);
    assert_eq!(parent.as_deref(), Some("epic"));
}
