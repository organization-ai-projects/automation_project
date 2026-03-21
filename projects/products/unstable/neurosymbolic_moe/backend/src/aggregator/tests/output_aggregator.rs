use crate::aggregator::{AggregationStrategy, OutputAggregator};
use crate::moe_core::{ExpertId, ExpertOutput};
use std::collections::HashMap;

fn expert_id(byte: u8) -> ExpertId {
    crate::tests::helpers::expert_id(byte)
}

fn make_output(id: u8, confidence: f64) -> ExpertOutput {
    ExpertOutput {
        expert_id: expert_id(id),
        content: format!("output-{id}"),
        confidence,
        metadata: HashMap::new(),
        trace: Vec::new(),
    }
}

#[test]
fn aggregate_highest_confidence() {
    let agg = OutputAggregator::new(AggregationStrategy::HighestConfidence);
    let outputs = vec![make_output(1, 0.7), make_output(2, 0.9)];
    let result = agg.aggregate(outputs).expect("aggregation should succeed");
    let selected = result
        .selected_output
        .expect("selected output should exist");
    assert_eq!(selected.expert_id, expert_id(2));
    assert_eq!(result.strategy, "highest_confidence");
}

#[test]
fn aggregate_first_strategy() {
    let agg = OutputAggregator::new(AggregationStrategy::First);
    let outputs = vec![make_output(1, 0.7), make_output(2, 0.9)];
    let result = agg.aggregate(outputs).expect("aggregation should succeed");
    let selected = result
        .selected_output
        .expect("selected output should exist");
    assert_eq!(selected.expert_id, expert_id(1));
    assert_eq!(result.strategy, "first");
}

#[test]
fn aggregate_empty_outputs_returns_error() {
    let agg = OutputAggregator::new(AggregationStrategy::HighestConfidence);
    let result = agg.aggregate(vec![]);
    assert!(result.is_err());
}
