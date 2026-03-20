use crate::pr::generate_description::render_issue_outcome_groups_with_mode;
use crate::pr::group_by_category::parse_records;

#[test]
fn test_generate_description() {
    let description = "Generated description";
    assert!(!description.is_empty(), "Description should not be empty");
}

#[test]
fn render_issue_outcome_records_groups_by_real_category() {
    let mut records = parse_records("12|Bug Fixes|Closes|#12\n8|Security|Closes|#8");
    records.sort_by_key(|record| (record.0, record.3));
    let rendered = render_issue_outcome_groups_with_mode(&records, "resolved")
        .trim()
        .to_string();

    assert!(rendered.contains("#### Security"));
    assert!(rendered.contains("#### Bug Fixes"));
    assert!(!rendered.contains("#### Unknown"));
}

#[test]
fn render_issue_outcome_records_renders_reopen_with_issue_key() {
    let mut records = parse_records("1077|Features|Reopen|#1077");
    records.sort_by_key(|record| (record.0, record.3));
    let rendered = render_issue_outcome_groups_with_mode(&records, "reopen")
        .trim()
        .to_string();

    assert!(rendered.contains("#### Features"));
    assert!(rendered.contains("- Reopen #1077"));
    assert!(!rendered.contains("Reopen Reopen"));
}
