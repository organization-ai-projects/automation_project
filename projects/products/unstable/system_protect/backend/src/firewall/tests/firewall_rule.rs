use crate::firewall::firewall_rule::FirewallRule;
use crate::moe_protect::protection_action::ProtectionAction;

#[test]
fn rule_matches_wildcard_source() {
    let rule = FirewallRule::new("test", "*", "target", ProtectionAction::Block, 1);
    assert!(rule.matches("any-source", "target"));
}

#[test]
fn rule_matches_specific_source_and_target() {
    let rule = FirewallRule::new("test", "192.168", "api/", ProtectionAction::Allow, 1);
    assert!(rule.matches("192.168.1.1", "api/users"));
}

#[test]
fn rule_does_not_match_wrong_target() {
    let rule = FirewallRule::new("test", "*", "specific-target", ProtectionAction::Block, 1);
    assert!(!rule.matches("source", "other-target"));
}
