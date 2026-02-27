use crate::decision_aggregator::{DecisionAggregatorConfig, aggregate};
use crate::domain::{DecisionContribution, FinalDecision};

fn contribution(id: &str, vote: FinalDecision, confidence: u8, weight: u8) -> DecisionContribution {
    DecisionContribution {
        contributor_id: id.to_string(),
        capability: "test".to_string(),
        vote,
        confidence,
        weight,
        reason_codes: Vec::new(),
        artifact_refs: Vec::new(),
    }
}

#[test]
fn aggregate_prefers_highest_weighted_score() {
    let summary = aggregate(
        &[
            contribution("a", FinalDecision::Proceed, 80, 80),
            contribution("b", FinalDecision::Block, 70, 40),
        ],
        &DecisionAggregatorConfig {
            min_confidence_to_proceed: 70,
        },
    );

    assert_eq!(summary.final_decision, FinalDecision::Proceed);
    assert!(summary.decision_confidence >= 70);
    assert!(summary.decision_rationale_codes.is_empty());
}

#[test]
fn aggregate_tie_uses_fail_closed_order() {
    let summary = aggregate(
        &[
            contribution("a", FinalDecision::Proceed, 50, 50),
            contribution("b", FinalDecision::Escalate, 50, 50),
            contribution("c", FinalDecision::Block, 50, 50),
        ],
        &DecisionAggregatorConfig {
            min_confidence_to_proceed: 70,
        },
    );

    assert_eq!(summary.final_decision, FinalDecision::Block);
    assert!(
        summary
            .decision_rationale_codes
            .contains(&"DECISION_TIE_FAIL_CLOSED".to_string())
    );
}

#[test]
fn aggregate_threshold_can_force_proceed_to_block() {
    let summary = aggregate(
        &[
            contribution("a", FinalDecision::Proceed, 55, 1),
            contribution("b", FinalDecision::Block, 45, 1),
        ],
        &DecisionAggregatorConfig {
            min_confidence_to_proceed: 70,
        },
    );

    assert_eq!(summary.final_decision, FinalDecision::Block);
    assert!(
        summary
            .decision_rationale_codes
            .contains(&"DECISION_CONFIDENCE_BELOW_THRESHOLD".to_string())
    );
}

#[test]
fn aggregate_empty_contributions_fails_closed() {
    let summary = aggregate(&[], &DecisionAggregatorConfig::default());

    assert_eq!(summary.final_decision, FinalDecision::Block);
    assert_eq!(summary.decision_confidence, 0);
    assert!(
        summary
            .decision_rationale_codes
            .contains(&"DECISION_NO_CONTRIBUTIONS".to_string())
    );
}
