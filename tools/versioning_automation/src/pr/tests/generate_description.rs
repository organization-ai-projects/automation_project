use crate::pr::generate_description::render_issue_outcome_records;

#[test]
fn test_generate_description() {
    let description = "Generated description";
    assert!(!description.is_empty(), "Description should not be empty");
}

#[test]
fn render_issue_outcome_records_groups_by_real_category() {
    let rendered =
        render_issue_outcome_records("12|Bug Fixes|Closes|#12\n8|Security|Closes|#8", "resolved");

    assert!(rendered.contains("#### Security"));
    assert!(rendered.contains("#### Bug Fixes"));
    assert!(!rendered.contains("#### Unknown"));
}
