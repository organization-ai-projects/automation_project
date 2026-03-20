use crate::moe_core::ExpertType;

#[test]
fn expert_type_variants_are_constructible() {
    let deterministic = ExpertType::Deterministic;
    let hybrid = ExpertType::Hybrid;
    assert!(matches!(deterministic, ExpertType::Deterministic));
    assert!(matches!(hybrid, ExpertType::Hybrid));
}
