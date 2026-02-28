use crate::decision_aggregator::{DecisionAggregatorConfig, aggregate};
use crate::domain::{DecisionContribution, DecisionReliabilityInput, FinalDecision};

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
            reliability_inputs: Vec::new(),
            memory_reliability_inputs: Vec::new(),
        },
    );

    assert_eq!(summary.final_decision, FinalDecision::Proceed);
    assert!(summary.decision_confidence >= 70);
    assert!(
        summary
            .decision_rationale_codes
            .contains(&"DECISION_RELIABILITY_COLD_START".to_string())
    );
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
            reliability_inputs: Vec::new(),
            memory_reliability_inputs: Vec::new(),
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
            reliability_inputs: Vec::new(),
            memory_reliability_inputs: Vec::new(),
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

#[test]
fn aggregate_cold_start_marks_reliability_cold_start() {
    let summary = aggregate(
        &[contribution("a", FinalDecision::Proceed, 80, 80)],
        &DecisionAggregatorConfig {
            min_confidence_to_proceed: 70,
            reliability_inputs: Vec::new(),
            memory_reliability_inputs: Vec::new(),
        },
    );

    assert!(
        summary
            .decision_rationale_codes
            .contains(&"DECISION_RELIABILITY_COLD_START".to_string())
    );
    assert_eq!(summary.reliability_factors.len(), 1);
    assert_eq!(summary.reliability_factors[0].reliability_score, 50);
}

#[test]
fn aggregate_reliability_drift_prefers_higher_reliability_contributor() {
    let summary = aggregate(
        &[
            contribution("a", FinalDecision::Proceed, 70, 50),
            contribution("b", FinalDecision::Block, 70, 50),
        ],
        &DecisionAggregatorConfig {
            min_confidence_to_proceed: 70,
            reliability_inputs: vec![
                DecisionReliabilityInput {
                    contributor_id: "a".to_string(),
                    capability: "test".to_string(),
                    score: 90,
                },
                DecisionReliabilityInput {
                    contributor_id: "b".to_string(),
                    capability: "test".to_string(),
                    score: 10,
                },
            ],
            memory_reliability_inputs: Vec::new(),
        },
    );

    assert_eq!(summary.final_decision, FinalDecision::Proceed);
    assert!(
        summary
            .decision_rationale_codes
            .contains(&"DECISION_RELIABILITY_WEIGHTED".to_string())
    );
    assert_eq!(summary.reliability_updates.len(), 2);
}

#[test]
fn aggregate_fail_closed_tie_still_applies_with_reliability() {
    let summary = aggregate(
        &[
            contribution("a", FinalDecision::Proceed, 50, 50),
            contribution("b", FinalDecision::Escalate, 50, 50),
            contribution("c", FinalDecision::Block, 50, 50),
        ],
        &DecisionAggregatorConfig {
            min_confidence_to_proceed: 70,
            reliability_inputs: vec![
                DecisionReliabilityInput {
                    contributor_id: "a".to_string(),
                    capability: "test".to_string(),
                    score: 50,
                },
                DecisionReliabilityInput {
                    contributor_id: "b".to_string(),
                    capability: "test".to_string(),
                    score: 50,
                },
                DecisionReliabilityInput {
                    contributor_id: "c".to_string(),
                    capability: "test".to_string(),
                    score: 50,
                },
            ],
            memory_reliability_inputs: Vec::new(),
        },
    );

    assert_eq!(summary.final_decision, FinalDecision::Block);
    assert!(
        summary
            .decision_rationale_codes
            .contains(&"DECISION_TIE_FAIL_CLOSED".to_string())
    );
}
