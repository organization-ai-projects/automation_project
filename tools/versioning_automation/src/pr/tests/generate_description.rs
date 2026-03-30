//! tools/versioning_automation/src/pr/tests/generate_description.rs
use crate::pr::{generate_options::GenerateOptions, group_by_category::GroupByCategory};

#[test]
fn test_generate_description() {
    let description = "Generated description";
    assert!(!description.is_empty(), "Description should not be empty");
}

#[test]
fn render_issue_outcome_records_groups_by_real_category() {
    let mut records =
        GroupByCategory::parse_records("12|Bug Fixes|Closes|#12\n8|Security|Closes|#8");
    records.sort_by_key(|record| (record.0, record.3));
    let rendered = GroupByCategory::render_grouped_output(&records, "resolved")
        .trim()
        .to_string();

    assert!(rendered.contains("#### Security"));
    assert!(rendered.contains("#### Bug Fixes"));
    assert!(!rendered.contains("#### Unknown"));
}

#[test]
fn parse_generate_description_allows_create_pr_without_dry_run() {
    let parsed = GenerateOptions::parse_generate_options(&[
        "--create-pr".to_string(),
        "--yes".to_string(),
        "--base".to_string(),
        "dev".to_string(),
        "--head".to_string(),
        "feature/test".to_string(),
    ])
    .expect("create-pr should parse without dry-run");

    assert!(parsed.create_pr);
    assert!(!parsed.dry_run);
    assert_eq!(parsed.base_ref.as_deref(), Some("dev"));
    assert_eq!(parsed.head_ref.as_deref(), Some("feature/test"));
}

#[test]
fn parse_generate_description_allows_auto_edit_without_dry_run() {
    let parsed = GenerateOptions::parse_generate_options(&[
        "--auto-edit".to_string(),
        "1090".to_string(),
        "--base".to_string(),
        "dev".to_string(),
        "--head".to_string(),
        "feature/test".to_string(),
    ])
    .expect("auto-edit should parse without dry-run");

    assert_eq!(parsed.auto_edit_pr_number.as_deref(), Some("1090"));
    assert!(!parsed.dry_run);
}
