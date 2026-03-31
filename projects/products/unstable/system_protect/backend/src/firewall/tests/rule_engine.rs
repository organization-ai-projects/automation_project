use crate::firewall::firewall_rule::FirewallRule;
use crate::firewall::rule_engine::RuleEngine;
use crate::moe_protect::protection_action::ProtectionAction;

#[test]
fn default_engine_has_rules() {
    let engine = RuleEngine::with_defaults();
    assert!(engine.rule_count() > 0);
}

#[test]
fn engine_returns_highest_priority_match() {
    let mut engine = RuleEngine::new();
    engine.add_rule(FirewallRule::new(
        "low",
        "*",
        "*",
        ProtectionAction::Log,
        10,
    ));
    engine.add_rule(FirewallRule::new(
        "high",
        "*",
        "*",
        ProtectionAction::Block,
        100,
    ));

    let result = engine.evaluate("any", "any");
    assert!(result.is_some());
    assert_eq!(result.unwrap().name, "high");
}

#[test]
fn engine_returns_none_when_no_match() {
    let engine = RuleEngine::new();
    let result = engine.evaluate("source", "target");
    assert!(result.is_none());
}
