use crate::pr::text_payload::extract_effective_action_issue_numbers;

#[test]
fn extract_effective_action_issue_numbers_ignores_part_of_refs() {
    let (closes, reopens) = extract_effective_action_issue_numbers("Part of #12\nPart of #34");
    assert!(closes.is_empty());
    assert!(reopens.is_empty());
}

#[test]
fn extract_effective_action_issue_numbers_keeps_effective_reopen() {
    let (closes, reopens) =
        extract_effective_action_issue_numbers("Closes #12\nCancel-Closes #12\nReopen #12");
    assert!(closes.is_empty());
    assert!(reopens.contains("12"));
}

#[test]
fn extract_effective_action_issue_numbers_keeps_later_close_after_reopen() {
    let (closes, reopens) = extract_effective_action_issue_numbers("Reopen #12\nCloses #12");
    assert!(closes.contains("12"));
    assert!(reopens.is_empty());
}
