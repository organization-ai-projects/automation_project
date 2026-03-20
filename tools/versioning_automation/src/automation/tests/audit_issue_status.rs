#[test]
fn extract_issue_refs_separates_closing_and_part_of_refs() {
    let text = "Closes #12\npart of #34\nFixes #56\nrelated to #78\nreopen #90";
    let (closing, reopen, part) =
        super::super::audit_issue_status::extract_issue_refs_from_text(text)
            .expect("refs should parse");

    assert!(closing.contains("12"));
    assert!(closing.contains("56"));
    assert!(reopen.contains("90"));
    assert!(part.contains("34"));
    assert!(part.contains("78"));
}

#[test]
fn extract_issue_refs_ignores_cancelled_closes() {
    let text = "Closes #12\nCancel-Closes #12\nReopen #12\nPart of #34";
    let (closing, reopen, part) =
        super::super::audit_issue_status::extract_issue_refs_from_text(text)
            .expect("refs should parse");

    assert!(!closing.contains("12"));
    assert!(reopen.contains("12"));
    assert!(part.contains("34"));
}

#[test]
fn extract_parent_field_normalizes_parent_value() {
    let body = "Some body\nParent: (EPIC)\nOther: value";
    let parent = super::super::audit_issue_status::extract_parent_field(body);
    assert_eq!(parent.as_deref(), Some("epic"));
}

#[test]
fn render_issue_audit_report_contains_summary_and_sections() {
    let report = super::super::audit_issue_status::render_issue_audit_report(
        "org/repo",
        "main..HEAD",
        3,
        &["- [#1](u) Done".to_string()],
        &["- [#2](u) Close".to_string()],
        &["- [#4](u) Reopen".to_string()],
        &[],
        &["- [#3](u) None".to_string()],
    );

    assert!(report.contains("# Issue Status Audit"));
    assert!(report.contains("- Open issues fetched: 3"));
    assert!(report.contains("## Done In Dev (Label)"));
    assert!(report.contains("## Would Close On Merge"));
    assert!(report.contains("## Would Reopen"));
    assert!(report.contains("## Part-Of Only"));
    assert!(report.contains("## Unreferenced"));
}
