//! projects/products/unstable/neurosymbolic_moe/backend/src/moe_core/tests/expert_metadata.rs
use crate::{
    moe_core::{self, ExpertCapability, ExpertMetadata, ExpertStatus, ExpertType},
    orchestrator::Version,
};

fn expert_id(byte: u8) -> moe_core::ExpertId {
    crate::tests::helpers::expert_id(byte)
}

#[test]
fn expert_metadata_fields_round_trip() {
    let metadata = ExpertMetadata {
        id: expert_id(1),
        name: "Expert".to_string(),
        version: Version::new(1, 0, 0),
        capabilities: vec![ExpertCapability::Routing],
        status: ExpertStatus::Active,
        expert_type: ExpertType::Deterministic,
    };
    assert_eq!(metadata.id, expert_id(1));
    assert_eq!(metadata.name, "Expert");
    assert_eq!(metadata.version, Version::new(1, 0, 0));
    assert_eq!(metadata.capabilities.len(), 1);
}
