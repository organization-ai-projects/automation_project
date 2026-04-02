use crate::pr::state::build_state;

#[test]
fn keeps_latest_explicit_decision_per_issue() {
    let text = "Directive Decision: #4 => close\nDirective Decision: #4 => reopen";
    let state = build_state(text);
    assert_eq!(
        state.explicit_decisions.get("#4").map(String::as_str),
        Some("reopen")
    );
}

#[test]
fn inferred_decision_honors_explicit() {
    let text = "Directive Decision: #4 => close\nReopen #4";
    let state = build_state(text);
    assert!(!state.inferred_decisions.contains_key("#4"));
}

#[test]
fn emits_deduped_actions() {
    let text = "Closes #2\nCloses #2\nReopen #2\nReopen #2\n#7 duplicate of #5\n#7 duplicate of #5";
    let state = build_state(text);
    assert_eq!(state.action_records.len(), 2);
    assert!(
        state
            .action_records
            .iter()
            .any(|record| record.first == "Reopen" && record.second == "#2")
    );
}

#[test]
fn cancel_closes_removes_inferred_close() {
    let text = "Closes #2\nCancel-Closes #2";
    let state = build_state(text);
    assert!(!state.inferred_decisions.contains_key("#2"));
    assert!(
        !state
            .action_records
            .iter()
            .any(|record| record.second == "#2")
    );
}

#[test]
fn cancel_closes_allows_later_close_to_win_again() {
    let text = "Closes #2\nCancel-Closes #2\nCloses #2";
    let state = build_state(text);
    assert_eq!(
        state.inferred_decisions.get("#2").map(String::as_str),
        Some("close")
    );
    assert!(
        state
            .action_records
            .iter()
            .any(|record| record.first == "Closes" && record.second == "#2")
    );
}
