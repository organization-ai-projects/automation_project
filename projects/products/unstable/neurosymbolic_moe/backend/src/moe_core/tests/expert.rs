use crate::moe_core::{ExpertCapability, ExpertId, ExpertMetadata, ExpertStatus, ExpertType};

#[test]
fn expert_id_and_metadata_creation() {
    let id = ExpertId::new("expert-1");
    let metadata = ExpertMetadata {
        id: id.clone(),
        name: "Expert One".to_string(),
        version: "1.0.0".to_string(),
        capabilities: vec![ExpertCapability::Routing, ExpertCapability::Retrieval],
        status: ExpertStatus::Active,
        expert_type: ExpertType::Hybrid,
    };

    assert_eq!(id.as_str(), "expert-1");
    assert_eq!(metadata.id.as_str(), "expert-1");
    assert_eq!(metadata.name, "Expert One");
    assert_eq!(metadata.version, "1.0.0");
    assert!(matches!(metadata.status, ExpertStatus::Active));
    assert!(matches!(metadata.expert_type, ExpertType::Hybrid));
    assert_eq!(metadata.capabilities.len(), 2);
}
