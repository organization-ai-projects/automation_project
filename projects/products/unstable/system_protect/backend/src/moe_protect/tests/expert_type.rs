use crate::moe_protect::expert_type::ExpertType;

#[test]
fn expert_types_are_distinct() {
    assert_ne!(ExpertType::Antivirus, ExpertType::Firewall);
    assert_ne!(ExpertType::SymbolicAnalyzer, ExpertType::Hybrid);
}
