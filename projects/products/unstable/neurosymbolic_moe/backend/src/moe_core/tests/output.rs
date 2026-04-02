//! projects/products/unstable/neurosymbolic_moe/backend/src/moe_core/tests/output.rs
use crate::moe_core::{self, AggregatedOutput, ExpertOutput};
use protocol::ProtocolId;
use std::collections::HashMap;
use std::str::FromStr;

fn expert_id(byte: u8) -> moe_core::ExpertId {
    moe_core::ExpertId::from_protocol_id(protocol_id(byte))
}

fn protocol_id(byte: u8) -> ProtocolId {
    ProtocolId::from_str(&format!("{:032x}", byte.max(1)))
        .expect("test protocol id should be valid fixed hex")
}

fn make_output(id: u8, content: &str, confidence: f64) -> ExpertOutput {
    ExpertOutput {
        expert_id: expert_id(id),
        content: content.to_string(),
        confidence,
        metadata: HashMap::new(),
        trace: Vec::new(),
    }
}

#[test]
fn expert_output_creation() {
    let out = make_output(1, "hello", 0.9);
    assert_eq!(out.expert_id, expert_id(1));
    assert_eq!(out.content, "hello");
    assert!((out.confidence - 0.9).abs() < f64::EPSILON);
}

#[test]
fn aggregated_output_creation() {
    let out = make_output(1, "hello", 0.9);
    let agg = AggregatedOutput {
        outputs: vec![out.clone()],
        selected_output: Some(out),
        strategy: "highest_confidence".to_string(),
    };
    assert_eq!(agg.outputs.len(), 1);
    assert!(agg.selected_output.is_some());
    assert_eq!(agg.strategy, "highest_confidence");
}
