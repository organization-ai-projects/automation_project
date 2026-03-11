use crate::moe_core::{ExpertId, ExpertOutput};
use std::collections::HashMap;

#[test]
fn expert_output_fields_round_trip() {
    let output = ExpertOutput {
        expert_id: ExpertId::new("e1"),
        content: "hello".to_string(),
        confidence: 0.9,
        metadata: HashMap::new(),
        trace: vec!["trace".to_string()],
    };
    assert_eq!(output.expert_id.as_str(), "e1");
    assert_eq!(output.content, "hello");
    assert_eq!(output.trace.len(), 1);
}
