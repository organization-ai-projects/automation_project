use crate::moe_protect::protection_action::ProtectionAction;
use crate::symbolic_engine::fact::Fact;
use crate::symbolic_engine::rule::SymbolicRule;

#[test]
fn symbolic_rule_has_conditions_and_action() {
    let rule = SymbolicRule::new(
        "test-rule",
        vec![Fact::new("a", "b", "c")],
        ProtectionAction::Block,
        0.9,
        "test reasoning",
    );
    assert_eq!(rule.name, "test-rule");
    assert_eq!(rule.conditions.len(), 1);
    assert_eq!(rule.conclusion_action, ProtectionAction::Block);
    assert!((rule.confidence - 0.9).abs() < f64::EPSILON);
}
