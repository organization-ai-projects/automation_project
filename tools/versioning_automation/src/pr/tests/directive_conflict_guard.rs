//! tools/versioning_automation/src/pr/tests/directive_conflict_guard.rs
use crate::pr::commands::PrDirectiveConflictGuardOptions;
use crate::pr::directive_conflict_guard::{build_directive_payload, run_directive_conflict_guard};
use crate::pr::state::build_state;

#[test]
fn directive_conflict_guard_returns_non_zero_when_pr_cannot_be_read() {
    let code = run_directive_conflict_guard(PrDirectiveConflictGuardOptions {
        pr_number: "".to_string(),
        repo: Some("organization/repository".to_string()),
    });
    assert_ne!(code, 0);
}

#[test]
fn directive_payload_keeps_branch_directives_after_body_text() {
    let payload = build_directive_payload("Reopen #12", "Closes #12");
    let state = build_state(&payload);
    assert!(
        state
            .action_records
            .iter()
            .any(|record| record.first == "Closes" && record.second == "#12")
    );
    assert!(
        !state
            .action_records
            .iter()
            .any(|record| record.first == "Reopen" && record.second == "#12")
    );
}
