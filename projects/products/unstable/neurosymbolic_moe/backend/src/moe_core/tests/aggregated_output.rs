//! projects/products/unstable/neurosymbolic_moe/backend/src/moe_core/tests/aggregated_output.rs
use crate::moe_core::{AggregatedOutput, ExpertId, ExpertOutput};
use std::collections::HashMap;

#[test]
fn aggregated_output_fields_round_trip() {
    let output = ExpertOutput {
        expert_id: ExpertId::new(),
        content: "content".to_string(),
        confidence: 0.8,
        metadata: HashMap::new(),
        trace: Vec::new(),
    };
    let aggregated = AggregatedOutput {
        outputs: vec![output.clone()],
        selected_output: Some(output),
        strategy: "first".to_string(),
    };
    assert_eq!(aggregated.outputs.len(), 1);
    assert_eq!(aggregated.strategy, "first");
    assert!(aggregated.selected_output.is_some());
}
