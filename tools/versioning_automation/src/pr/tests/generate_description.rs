//! tools/versioning_automation/src/pr/tests/generate_description.rs
use crate::pr::generate_description::render_issue_outcome_groups_with_mode;
use crate::pr::generate_options::GenerateOptions;
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
