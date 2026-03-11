use crate::aggregator::{AggregationStrategy, OutputAggregator};
use crate::moe_core::{ExpertId, ExpertOutput};
use std::collections::HashMap;

fn make_output(id: &str, confidence: f64) -> ExpertOutput {
    ExpertOutput {
        expert_id: ExpertId::new(id),
        content: format!("output-{id}"),
        confidence,
        metadata: HashMap::new(),
        trace: Vec::new(),
    }
}

#[test]
fn aggregate_highest_confidence() {
    let agg = OutputAggregator::new(AggregationStrategy::HighestConfidence);
    let outputs = vec![make_output("e1", 0.7), make_output("e2", 0.9)];
    let result = agg.aggregate(outputs).expect("aggregation should succeed");
    let selected = result
        .selected_output
        .expect("selected output should exist");
    assert_eq!(selected.expert_id.as_str(), "e2");
    assert_eq!(result.strategy, "highest_confidence");
}

#[test]
fn aggregate_first_strategy() {
    let agg = OutputAggregator::new(AggregationStrategy::First);
    let outputs = vec![make_output("e1", 0.7), make_output("e2", 0.9)];
    let result = agg.aggregate(outputs).expect("aggregation should succeed");
    let selected = result
        .selected_output
        .expect("selected output should exist");
    assert_eq!(selected.expert_id.as_str(), "e1");
    assert_eq!(result.strategy, "first");
}

#[test]
fn aggregate_empty_outputs_returns_error() {
    let agg = OutputAggregator::new(AggregationStrategy::HighestConfidence);
    let result = agg.aggregate(vec![]);
    assert!(result.is_err());
}
