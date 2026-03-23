//! projects/products/unstable/neurosymbolic_moe/backend/src/moe_core/tests/expert_output.rs
use protocol::ProtocolId;
use std::collections::HashMap;
use std::str::FromStr;

use crate::moe_core::{self, ExpertOutput};

fn expert_id(byte: u8) -> moe_core::ExpertId {
    moe_core::ExpertId::from_protocol_id(protocol_id(byte))
}

fn protocol_id(byte: u8) -> ProtocolId {
    ProtocolId::from_str(&format!("{:032x}", byte.max(1)))
        .expect("test protocol id should be valid fixed hex")
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
