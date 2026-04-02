use crate::moe_core::ExpertStatus;

#[test]
fn expert_status_variants_are_constructible() {
    let active = ExpertStatus::Active;
    let inactive = ExpertStatus::Inactive;
    assert!(matches!(active, ExpertStatus::Active));
    assert!(matches!(inactive, ExpertStatus::Inactive));
}
