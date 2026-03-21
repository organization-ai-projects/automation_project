//! projects/products/unstable/neurosymbolic_moe/backend/src/moe_core/tests/expert_output.rs
use std::collections::HashMap;

use crate::moe_core::{self, ExpertOutput};

fn expert_id(byte: u8) -> moe_core::ExpertId {
    crate::tests::helpers::expert_id(byte)
}

#[test]
fn expert_output_fields_round_trip() {
    let output = ExpertOutput {
        expert_id: expert_id(1),
        content: "hello".to_string(),
        confidence: 0.9,
        metadata: HashMap::new(),
        trace: vec!["trace".to_string()],
    };
    assert_eq!(output.expert_id, expert_id(1));
    assert_eq!(output.content, "hello");
    assert_eq!(output.trace.len(), 1);
}
