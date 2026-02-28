// projects/products/unstable/autonomy_orchestrator_ai/src/review_ensemble.rs
use crate::domain::{ReviewEnsembleResult, ReviewVerdict, ReviewerVerdict};

pub const MANDATORY_SPECIALTIES: &[&str] = &["correctness", "security", "maintainability"];

pub const REASON_TIE_FAIL_CLOSED: &str = "REVIEW_ENSEMBLE_TIE_FAIL_CLOSED";
pub const REASON_CONFIDENCE_BELOW_THRESHOLD: &str = "REVIEW_ENSEMBLE_CONFIDENCE_BELOW_THRESHOLD";
pub const REASON_SECURITY_REJECTION: &str = "REVIEW_ENSEMBLE_SECURITY_REJECTION";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewEnsembleConfig {
    pub min_approval_confidence: u8,
}

impl Default for ReviewEnsembleConfig {
    fn default() -> Self {
        Self {
            min_approval_confidence: 70,
        }
    }
}

/// Returns the mandatory specialties that have no corresponding verdict in `verdicts`.
pub fn missing_mandatory_specialties<'a>(verdicts: &[ReviewerVerdict]) -> Vec<&'a str> {
    MANDATORY_SPECIALTIES
        .iter()
        .filter(|&&specialty| !verdicts.iter().any(|v| v.specialty == specialty))
        .copied()
        .collect()
}

pub fn run_review_ensemble(
    verdicts: &[ReviewerVerdict],
    cfg: &ReviewEnsembleConfig,
) -> ReviewEnsembleResult {
    if verdicts.is_empty() {
        return ReviewEnsembleResult {
            passed: false,
            confidence: 0,
            reason_codes: vec![REASON_TIE_FAIL_CLOSED.to_string()],
        };
    }

    // Security rejection is non-bypassable: any security reviewer rejecting blocks immediately.
    let has_security_rejection = verdicts
        .iter()
        .any(|v| v.specialty == "security" && v.verdict == ReviewVerdict::Reject);
    if has_security_rejection {
        return ReviewEnsembleResult {
            passed: false,
            confidence: 100,
            reason_codes: vec![REASON_SECURITY_REJECTION.to_string()],
        };
    }

    // Deterministic weighted vote: score = confidence * weight.
    let mut approve_score = 0u64;
    let mut reject_score = 0u64;
    for verdict in verdicts {
        let score = u64::from(verdict.confidence) * u64::from(verdict.weight);
        match verdict.verdict {
            ReviewVerdict::Approve => approve_score += score,
            ReviewVerdict::Reject => reject_score += score,
        }
    }

    let total_score = approve_score + reject_score;

    // Tie => fail-closed blocked.
    if approve_score == reject_score {
        return ReviewEnsembleResult {
            passed: false,
            confidence: if total_score == 0 { 0 } else { 50 },
            reason_codes: vec![REASON_TIE_FAIL_CLOSED.to_string()],
        };
    }

    let passed = approve_score > reject_score;
    let winner_score = if passed { approve_score } else { reject_score };
    let confidence = if total_score == 0 {
        0
    } else {
        let numerator = winner_score.saturating_mul(100);
        let rounded = numerator + (total_score / 2);
        (rounded / total_score).min(100) as u8
    };

    // Approval requires confidence floor.
    if passed && confidence < cfg.min_approval_confidence {
        return ReviewEnsembleResult {
            passed: false,
            confidence,
            reason_codes: vec![REASON_CONFIDENCE_BELOW_THRESHOLD.to_string()],
        };
    }

    ReviewEnsembleResult {
        passed,
        confidence,
        reason_codes: Vec::new(),
    }
}
