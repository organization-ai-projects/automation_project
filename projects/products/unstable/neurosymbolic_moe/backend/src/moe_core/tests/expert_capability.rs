use crate::moe_core::ExpertCapability;

#[test]
fn expert_capability_variants_are_constructible() {
    let generation = ExpertCapability::CodeGeneration;
    let custom = ExpertCapability::Custom("x".to_string());
    assert!(matches!(generation, ExpertCapability::CodeGeneration));
    assert!(matches!(custom, ExpertCapability::Custom(_)));
}
