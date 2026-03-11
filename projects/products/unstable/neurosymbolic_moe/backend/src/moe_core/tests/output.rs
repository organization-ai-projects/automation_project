use crate::moe_core::{AggregatedOutput, ExpertId, ExpertOutput};
use std::collections::HashMap;

fn make_output(id: &str, content: &str, confidence: f64) -> ExpertOutput {
    ExpertOutput {
        expert_id: ExpertId::new(id),
        content: content.to_string(),
        confidence,
        metadata: HashMap::new(),
        trace: Vec::new(),
    }
}

#[test]
fn expert_output_creation() {
    let out = make_output("e1", "hello", 0.9);
    assert_eq!(out.expert_id.as_str(), "e1");
    assert_eq!(out.content, "hello");
    assert!((out.confidence - 0.9).abs() < f64::EPSILON);
}

#[test]
fn aggregated_output_creation() {
    let out = make_output("e1", "hello", 0.9);
    let agg = AggregatedOutput {
        outputs: vec![out.clone()],
        selected_output: Some(out),
        strategy: "highest_confidence".to_string(),
    };
    assert_eq!(agg.outputs.len(), 1);
    assert!(agg.selected_output.is_some());
    assert_eq!(agg.strategy, "highest_confidence");
}
