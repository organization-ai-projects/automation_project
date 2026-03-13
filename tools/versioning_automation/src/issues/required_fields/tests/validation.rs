#[test]
fn validation_as_pipe_line_formats_all_fields() {
    let validation = crate::issues::required_fields::validation::Validation::new(
        "missing_field",
        "Parent".to_string(),
        "Missing required field: Parent:".to_string(),
    );
    assert_eq!(
        validation.as_pipe_line(),
        "missing_field|Parent|Missing required field: Parent:"
    );
}
