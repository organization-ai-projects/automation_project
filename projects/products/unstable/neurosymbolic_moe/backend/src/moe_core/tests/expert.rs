//! projects/products/unstable/neurosymbolic_moe/backend/src/moe_core/tests/expert.rs
use crate::{
    moe_core::{self, ExpertCapability, ExpertMetadata, ExpertStatus, ExpertType},
    orchestrator::Version,
};

fn expert_id(byte: u8) -> moe_core::ExpertId {
    crate::tests::helpers::expert_id(byte)
}

#[test]
fn expert_id_and_metadata_creation() {
    let id = expert_id(1);
    let metadata = ExpertMetadata {
        id: id.clone(),
        name: "Expert One".to_string(),
        version: Version::new(1, 0, 0),
        capabilities: vec![ExpertCapability::Routing, ExpertCapability::Retrieval],
        status: ExpertStatus::Active,
        expert_type: ExpertType::Hybrid,
    };

    assert_eq!(id, expert_id(1));
    assert_eq!(metadata.id, expert_id(1));
    assert_eq!(metadata.name, "Expert One");
    assert_eq!(metadata.version, Version::new(1, 0, 0));
    assert!(matches!(metadata.status, ExpertStatus::Active));
    assert!(matches!(metadata.expert_type, ExpertType::Hybrid));
    assert_eq!(metadata.capabilities.len(), 2);
}
