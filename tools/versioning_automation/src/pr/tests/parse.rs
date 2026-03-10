use crate::pr::run;

#[test]
fn pr_help_returns_zero() {
    let args = vec!["help".to_string()];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_breaking_detect_requires_input() {
    let args = vec!["breaking-detect".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_body_context_requires_pr() {
    let args = vec!["body-context".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_body_context_with_required_fields_returns_zero() {
    let args = vec![
        "body-context".to_string(),
        "--pr".to_string(),
        "42".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_child_pr_refs_requires_pr() {
    let args = vec!["child-pr-refs".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_child_pr_refs_with_required_fields_returns_zero() {
    let args = vec![
        "child-pr-refs".to_string(),
        "--pr".to_string(),
        "42".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_details_requires_pr() {
    let args = vec!["details".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_details_with_required_fields_returns_zero() {
    let args = vec!["details".to_string(), "--pr".to_string(), "42".to_string()];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_breaking_detect_with_text_returns_zero() {
    let args = vec![
        "breaking-detect".to_string(),
        "--text".to_string(),
        "- [x] Breaking change".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_breaking_detect_with_input_file_returns_zero() {
    let file_path = "/tmp/va_pr_breaking_detect_input.txt";
    std::fs::write(file_path, "- [x] Breaking change").expect("write input file");
    let args = vec![
        "breaking-detect".to_string(),
        "--input-file".to_string(),
        file_path.to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
    std::fs::remove_file(file_path).expect("remove input file");
}

#[test]
fn pr_directives_requires_input() {
    let args = vec!["directives".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_directives_apply_requires_input() {
    let args = vec!["directives-apply".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_directives_apply_with_text_returns_zero() {
    let args = vec![
        "directives-apply".to_string(),
        "--text".to_string(),
        "Closes #1".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_directives_with_text_returns_zero() {
    let args = vec![
        "directives".to_string(),
        "--text".to_string(),
        "Closes #1".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_directives_with_input_file_returns_zero() {
    let file_path = "/tmp/va_pr_directives_input.txt";
    std::fs::write(file_path, "Closes #12").expect("write input file");
    let args = vec![
        "directives".to_string(),
        "--input-file".to_string(),
        file_path.to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
    std::fs::remove_file(file_path).expect("remove input file");
}

#[test]
fn pr_directives_with_missing_input_file_returns_error() {
    let args = vec![
        "directives".to_string(),
        "--input-file".to_string(),
        "/tmp/va_pr_missing_input_file.txt".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_auto_add_closes_requires_pr_flag() {
    let args = vec!["auto-add-closes".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_directive_conflicts_requires_input() {
    let args = vec!["directive-conflicts".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_directive_conflicts_with_text_returns_zero() {
    let args = vec![
        "directive-conflicts".to_string(),
        "--text".to_string(),
        "Closes #1\nReopen #1".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_directive_conflict_guard_requires_pr_flag() {
    let args = vec!["directive-conflict-guard".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_duplicate_actions_requires_input() {
    let args = vec!["duplicate-actions".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_duplicate_actions_requires_mode_and_repo() {
    let args = vec![
        "duplicate-actions".to_string(),
        "--text".to_string(),
        "#2|#1".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_group_by_category_requires_input() {
    let args = vec!["group-by-category".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_group_by_category_requires_mode() {
    let args = vec![
        "group-by-category".to_string(),
        "--text".to_string(),
        "1|Bug Fixes|Closes|#1".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_issue_ref_kind_requires_issue() {
    let args = vec!["issue-ref-kind".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_issue_context_requires_issue() {
    let args = vec!["issue-context".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_issue_context_with_required_fields_returns_zero() {
    let args = vec![
        "issue-context".to_string(),
        "--issue".to_string(),
        "42".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_issue_view_requires_issue() {
    let args = vec!["issue-view".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_issue_view_with_required_fields_returns_zero() {
    let args = vec![
        "issue-view".to_string(),
        "--issue".to_string(),
        "42".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_pr_state_requires_pr() {
    let args = vec!["pr-state".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_pr_state_with_required_fields_returns_zero() {
    let args = vec!["pr-state".to_string(), "--pr".to_string(), "42".to_string()];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_text_payload_requires_pr() {
    let args = vec!["text-payload".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_text_payload_with_required_fields_returns_zero() {
    let args = vec![
        "text-payload".to_string(),
        "--pr".to_string(),
        "42".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_open_referencing_issue_requires_issue() {
    let args = vec!["open-referencing-issue".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_open_referencing_issue_with_required_fields_returns_zero() {
    let args = vec![
        "open-referencing-issue".to_string(),
        "--issue".to_string(),
        "42".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_normalize_issue_key_requires_raw() {
    let args = vec!["normalize-issue-key".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_sort_bullets_requires_input_file() {
    let args = vec!["sort-bullets".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_issue_category_from_labels_requires_labels_raw() {
    let args = vec!["issue-category-from-labels".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_issue_category_from_title_requires_title() {
    let args = vec!["issue-category-from-title".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_issue_close_policy_requires_action() {
    let args = vec!["issue-close-policy".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_issue_close_policy_with_required_fields_returns_zero() {
    let args = vec![
        "issue-close-policy".to_string(),
        "--action".to_string(),
        "Closes".to_string(),
        "--is-pr-ref".to_string(),
        "false".to_string(),
        "--non-compliance-reason".to_string(),
        "reason".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_effective_category_requires_fields() {
    let args = vec!["effective-category".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_effective_category_accepts_title_category() {
    let args = vec![
        "effective-category".to_string(),
        "--labels-raw".to_string(),
        "automation||bug".to_string(),
        "--title-category".to_string(),
        "Unknown".to_string(),
        "--default-category".to_string(),
        "Mixed".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_directives_state_requires_input() {
    let args = vec!["directives-state".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_directives_state_with_text_returns_zero() {
    let args = vec![
        "directives-state".to_string(),
        "--text".to_string(),
        "Closes #1".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_closure_refs_requires_input() {
    let args = vec!["closure-refs".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_closure_refs_with_text_returns_zero() {
    let args = vec![
        "closure-refs".to_string(),
        "--text".to_string(),
        "Closes #1".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_issue_decision_requires_minimum_fields() {
    let args = vec!["issue-decision".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_issue_decision_with_required_fields_returns_zero() {
    let args = vec![
        "issue-decision".to_string(),
        "--action".to_string(),
        "Closes".to_string(),
        "--issue".to_string(),
        "#42".to_string(),
        "--default-category".to_string(),
        "Mixed".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_closure_marker_requires_required_flags() {
    let args = vec!["closure-marker".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_closure_marker_with_required_flags_returns_zero() {
    let args = vec![
        "closure-marker".to_string(),
        "--text".to_string(),
        "Closes #42".to_string(),
        "--keyword-pattern".to_string(),
        "closes".to_string(),
        "--issue".to_string(),
        "#42".to_string(),
        "--mode".to_string(),
        "apply".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_resolve_category_requires_flags() {
    let args = vec!["resolve-category".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_resolve_category_with_required_flags_returns_zero() {
    let args = vec![
        "resolve-category".to_string(),
        "--label-category".to_string(),
        "Unknown".to_string(),
        "--title-category".to_string(),
        "UI".to_string(),
        "--default-category".to_string(),
        "Mixed".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}

#[test]
fn pr_non_closing_refs_requires_input() {
    let args = vec!["non-closing-refs".to_string()];
    let code = run(&args);
    assert_eq!(code, 2);
}

#[test]
fn pr_non_closing_refs_with_text_returns_zero() {
    let args = vec![
        "non-closing-refs".to_string(),
        "--text".to_string(),
        "Part of #7".to_string(),
    ];
    let code = run(&args);
    assert_eq!(code, 0);
}
