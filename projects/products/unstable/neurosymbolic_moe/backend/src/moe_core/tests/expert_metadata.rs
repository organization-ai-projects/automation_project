use crate::moe_core::{ExpertCapability, ExpertId, ExpertMetadata, ExpertStatus, ExpertType};

#[test]
fn expert_metadata_fields_round_trip() {
    let metadata = ExpertMetadata {
        id: ExpertId::new("e1"),
        name: "Expert".to_string(),
        version: "1.0.0".to_string(),
        capabilities: vec![ExpertCapability::Routing],
        status: ExpertStatus::Active,
        expert_type: ExpertType::Deterministic,
    };
    assert_eq!(metadata.id.as_str(), "e1");
    assert_eq!(metadata.name, "Expert");
    assert_eq!(metadata.version, "1.0.0");
    assert_eq!(metadata.capabilities.len(), 1);
}
