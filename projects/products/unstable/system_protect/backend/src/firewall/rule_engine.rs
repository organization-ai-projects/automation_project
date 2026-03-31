use super::firewall_rule::FirewallRule;
use crate::moe_protect::protection_action::ProtectionAction;

pub struct RuleEngine {
    rules: Vec<FirewallRule>,
}

impl RuleEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn with_defaults() -> Self {
        let mut engine = Self::new();
        engine.add_rule(FirewallRule::new(
            "block-known-malicious",
            "10.0.0.0/8",
            "*.exe",
            ProtectionAction::Block,
            100,
        ));
        engine.add_rule(FirewallRule::new(
            "allow-internal",
            "192.168.*",
            "192.168.*",
            ProtectionAction::Allow,
            50,
        ));
        engine.add_rule(FirewallRule::new(
            "rate-limit-external",
            "*",
            "api/*",
            ProtectionAction::RateLimit,
            30,
        ));
        engine
    }

    pub fn add_rule(&mut self, rule: FirewallRule) {
        self.rules.push(rule);
        self.rules.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    pub fn evaluate(&self, source: &str, target: &str) -> Option<&FirewallRule> {
        self.rules.iter().find(|rule| rule.matches(source, target))
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}
