use crate::domain::{ReviewVerdict, ReviewerVerdict};
use crate::review_ensemble::{
    MANDATORY_SPECIALTIES, REASON_CONFIDENCE_BELOW_THRESHOLD, REASON_SECURITY_REJECTION,
    REASON_TIE_FAIL_CLOSED, ReviewEnsembleConfig, missing_mandatory_specialties,
    run_review_ensemble,
};

fn verdict(
    reviewer_id: &str,
    specialty: &str,
    verdict: ReviewVerdict,
    confidence: u8,
    weight: u8,
) -> ReviewerVerdict {
    ReviewerVerdict {
        reviewer_id: reviewer_id.to_string(),
        specialty: specialty.to_string(),
        verdict,
        confidence,
        weight,
        reason_codes: Vec::new(),
    }
}

#[test]
fn deterministic_weighted_majority_approve() {
    let cfg = ReviewEnsembleConfig::default();
    let result = run_review_ensemble(
        &[
            verdict("r1", "correctness", ReviewVerdict::Approve, 90, 80),
            verdict("r2", "security", ReviewVerdict::Approve, 85, 80),
            verdict("r3", "maintainability", ReviewVerdict::Reject, 70, 20),
        ],
        &cfg,
    );

    assert!(result.passed);
    assert!(result.reason_codes.is_empty());
}

#[test]
fn deterministic_weighted_majority_reject() {
    let cfg = ReviewEnsembleConfig::default();
    let result = run_review_ensemble(
        &[
            verdict("r1", "correctness", ReviewVerdict::Approve, 50, 30),
            verdict("r2", "security", ReviewVerdict::Approve, 50, 20),
            verdict("r3", "maintainability", ReviewVerdict::Reject, 90, 80),
        ],
        &cfg,
    );

    assert!(!result.passed);
    assert!(result.reason_codes.is_empty());
}

#[test]
fn tie_fails_closed_with_reason_code() {
    let cfg = ReviewEnsembleConfig::default();
    let result = run_review_ensemble(
        &[
            verdict("r1", "correctness", ReviewVerdict::Approve, 80, 50),
            verdict("r2", "maintainability", ReviewVerdict::Reject, 80, 50),
        ],
        &cfg,
    );

    assert!(!result.passed);
    assert!(
        result
            .reason_codes
            .contains(&REASON_TIE_FAIL_CLOSED.to_string())
    );
}

#[test]
fn empty_verdicts_fails_closed() {
    let cfg = ReviewEnsembleConfig::default();
    let result = run_review_ensemble(&[], &cfg);

    assert!(!result.passed);
    assert!(
        result
            .reason_codes
            .contains(&REASON_TIE_FAIL_CLOSED.to_string())
    );
}

#[test]
fn security_rejection_is_non_bypassable() {
    let cfg = ReviewEnsembleConfig::default();
    // Even with overwhelming approve votes, security reject blocks.
    let result = run_review_ensemble(
        &[
            verdict("r1", "correctness", ReviewVerdict::Approve, 100, 100),
            verdict("r2", "maintainability", ReviewVerdict::Approve, 100, 100),
            verdict("r3", "security", ReviewVerdict::Reject, 10, 1),
        ],
        &cfg,
    );

    assert!(!result.passed);
    assert!(
        result
            .reason_codes
            .contains(&REASON_SECURITY_REJECTION.to_string())
    );
}

#[test]
fn confidence_below_threshold_blocks_despite_approve_majority() {
    let cfg = ReviewEnsembleConfig {
        min_approval_confidence: 80,
    };
    // Approve wins the vote but confidence is below floor.
    let result = run_review_ensemble(
        &[
            verdict("r1", "correctness", ReviewVerdict::Approve, 55, 60),
            verdict("r2", "maintainability", ReviewVerdict::Reject, 40, 40),
        ],
        &cfg,
    );

    assert!(!result.passed);
    assert!(
        result
            .reason_codes
            .contains(&REASON_CONFIDENCE_BELOW_THRESHOLD.to_string())
    );
}

#[test]
fn security_approval_does_not_block() {
    let cfg = ReviewEnsembleConfig::default();
    let result = run_review_ensemble(
        &[
            verdict("r1", "correctness", ReviewVerdict::Approve, 90, 70),
            verdict("r2", "security", ReviewVerdict::Approve, 85, 80),
            verdict("r3", "maintainability", ReviewVerdict::Approve, 80, 60),
        ],
        &cfg,
    );

    assert!(result.passed);
    assert!(result.reason_codes.is_empty());
}

#[test]
fn arbitration_is_deterministic_for_same_input() {
    let cfg = ReviewEnsembleConfig::default();
    let verdicts = vec![
        verdict("r1", "correctness", ReviewVerdict::Approve, 75, 70),
        verdict("r2", "security", ReviewVerdict::Approve, 80, 80),
        verdict("r3", "maintainability", ReviewVerdict::Reject, 70, 50),
    ];
    let result_a = run_review_ensemble(&verdicts, &cfg);
    let result_b = run_review_ensemble(&verdicts, &cfg);

    assert_eq!(result_a, result_b);
}

#[test]
fn mandatory_specialties_covers_all_three() {
    assert_eq!(MANDATORY_SPECIALTIES.len(), 3);
    assert!(MANDATORY_SPECIALTIES.contains(&"correctness"));
    assert!(MANDATORY_SPECIALTIES.contains(&"security"));
    assert!(MANDATORY_SPECIALTIES.contains(&"maintainability"));
}

#[test]
fn missing_mandatory_specialties_detects_absent_entries() {
    let verdicts = vec![
        verdict("r1", "correctness", ReviewVerdict::Approve, 80, 70),
        verdict("r2", "security", ReviewVerdict::Approve, 80, 70),
    ];
    let missing = missing_mandatory_specialties(&verdicts);
    assert_eq!(missing, vec!["maintainability"]);
}

#[test]
fn missing_mandatory_specialties_returns_empty_when_all_covered() {
    let verdicts = vec![
        verdict("r1", "correctness", ReviewVerdict::Approve, 80, 70),
        verdict("r2", "security", ReviewVerdict::Approve, 80, 70),
        verdict("r3", "maintainability", ReviewVerdict::Approve, 80, 70),
    ];
    let missing = missing_mandatory_specialties(&verdicts);
    assert!(missing.is_empty());
}
