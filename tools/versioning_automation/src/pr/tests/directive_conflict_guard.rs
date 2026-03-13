use crate::pr::commands::pr_directive_conflict_guard_options::PrDirectiveConflictGuardOptions;
use crate::pr::directive_conflict_guard::run_directive_conflict_guard;

#[test]
fn directive_conflict_guard_returns_non_zero_when_pr_cannot_be_read() {
    let code = run_directive_conflict_guard(PrDirectiveConflictGuardOptions {
        pr_number: "".to_string(),
        repo: Some("organization/repository".to_string()),
    });
    assert_ne!(code, 0);
}
