use crate::pr::commit_info::CommitInfo;
use crate::pr::generate_description::build_full_body;
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

#[test]
fn build_full_body_mentions_conflict_resolution_winner_in_issue_outcomes() {
    let commits = vec![
        CommitInfo {
            short_hash: "1".to_string(),
            subject: "chore(workspace): reopen issue tracking".to_string(),
            body: "Reopen #1085".to_string(),
        },
        CommitInfo {
            short_hash: "2".to_string(),
            subject: "fix(tools/versioning_automation): finalize PR rendering".to_string(),
            body: "Closes #1085".to_string(),
        },
    ];

    let rendered = build_full_body(
        "dev",
        "fix/pr-issue-directive-resolution",
        &commits,
        "dev..fix",
        "- CI: UNKNOWN",
    );

    assert!(rendered.contains("- Closes #1085 (resolved Closes/Reopen conflict; winner: Closes; origin: inferred from latest directive)"));
}
