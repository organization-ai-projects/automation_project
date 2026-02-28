// projects/products/unstable/autonomy_orchestrator_ai/src/pr_risk.rs
use crate::domain::{PrRiskBreakdown, PrRiskFactor, RunReport, Stage, StageExecutionStatus, TerminalState};

/// Computes a deterministic PR risk breakdown from the run report.
///
/// Risk factors:
/// - `gate_quality`: 12 points per non-passing gate decision (max 36)
/// - `decision_confidence`: 0/8/16/24 based on reported confidence tier
/// - `risk_tier`: 0/8/16 derived from terminal state (done/blocked/failed)
/// - `hard_gate_status`: 12 points if any hard gate blocking code is present
/// - `test_stability`: 0/6/12 based on number of validation-stage failures
///
/// `eligible_for_auto_merge` is `true` when `total_score <= threshold`.
pub fn compute_pr_risk(report: &RunReport, threshold: u16) -> PrRiskBreakdown {
    let mut factors = Vec::new();
    let mut total: u16 = 0;

    // Factor 1: gate_quality
    let failed_gates = report.gate_decisions.iter().filter(|d| !d.passed).count();
    let gate_score = (failed_gates as u16).saturating_mul(12);
    factors.push(PrRiskFactor {
        name: "gate_quality".to_string(),
        score: gate_score,
        rationale: format!("{failed_gates} gate(s) non-passing"),
    });
    total = total.saturating_add(gate_score);

    // Factor 2: decision_confidence
    let confidence_score: u16 = match report.decision_confidence {
        Some(c) if c >= 80 => 0,
        Some(c) if c >= 60 => 8,
        Some(c) if c >= 40 => 16,
        Some(_) => 24,
        None => 0,
    };
    let confidence_label = report
        .decision_confidence
        .map(|c| c.to_string())
        .unwrap_or_else(|| "none".to_string());
    factors.push(PrRiskFactor {
        name: "decision_confidence".to_string(),
        score: confidence_score,
        rationale: format!("decision confidence: {confidence_label}"),
    });
    total = total.saturating_add(confidence_score);

    // Factor 3: risk_tier (derived from terminal state)
    let (risk_tier_score, risk_tier_label): (u16, &str) = match report.terminal_state {
        Some(TerminalState::Done) => (0, "low"),
        Some(TerminalState::Blocked) => (8, "medium"),
        _ => (16, "high"),
    };
    factors.push(PrRiskFactor {
        name: "risk_tier".to_string(),
        score: risk_tier_score,
        rationale: format!("risk tier: {risk_tier_label}"),
    });
    total = total.saturating_add(risk_tier_score);

    // Factor 4: hard_gate_status
    const HARD_GATE_CODES: &[&str] = &[
        "GATE_POLICY_DENIED_OR_UNKNOWN",
        "GATE_CI_NOT_SUCCESS",
        "GATE_REVIEW_NOT_APPROVED",
    ];
    let hard_gate_blocked = report
        .blocked_reason_codes
        .iter()
        .any(|c| HARD_GATE_CODES.contains(&c.as_str()));
    let hard_gate_score: u16 = if hard_gate_blocked { 12 } else { 0 };
    factors.push(PrRiskFactor {
        name: "hard_gate_status".to_string(),
        score: hard_gate_score,
        rationale: if hard_gate_blocked {
            "hard gates blocked".to_string()
        } else {
            "hard gates clear".to_string()
        },
    });
    total = total.saturating_add(hard_gate_score);

    // Factor 5: test_stability
    let validation_failures = report
        .stage_executions
        .iter()
        .filter(|e| {
            e.stage == Stage::Validation
                && matches!(
                    e.status,
                    StageExecutionStatus::Failed
                        | StageExecutionStatus::Timeout
                        | StageExecutionStatus::SpawnFailed
                )
        })
        .count();
    let test_stability_score: u16 = if validation_failures == 0 {
        0
    } else if validation_failures < 3 {
        6
    } else {
        12
    };
    factors.push(PrRiskFactor {
        name: "test_stability".to_string(),
        score: test_stability_score,
        rationale: format!("{validation_failures} validation failure(s)"),
    });
    total = total.saturating_add(test_stability_score);

    let eligible_for_auto_merge = total <= threshold;

    PrRiskBreakdown {
        total_score: total,
        factors,
        eligible_for_auto_merge,
    }
}
