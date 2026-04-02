use serde::{Deserialize, Serialize};

use crate::moe_protect::protection_action::ProtectionAction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallRule {
    pub name: String,
    pub source_pattern: String,
    pub target_pattern: String,
    pub action: ProtectionAction,
    pub priority: u32,
}

impl FirewallRule {
    pub fn new(
        name: impl Into<String>,
        source_pattern: impl Into<String>,
        target_pattern: impl Into<String>,
        action: ProtectionAction,
        priority: u32,
    ) -> Self {
        Self {
            name: name.into(),
            source_pattern: source_pattern.into(),
            target_pattern: target_pattern.into(),
            action,
            priority,
        }
    }

    pub fn matches(&self, source: &str, target: &str) -> bool {
        let source_match = self.source_pattern == "*" || source.contains(&self.source_pattern);
        let target_match = self.target_pattern == "*" || target.contains(&self.target_pattern);
        source_match && target_match
    }
}
