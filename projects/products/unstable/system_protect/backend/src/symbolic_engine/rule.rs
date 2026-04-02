use serde::{Deserialize, Serialize};

use super::fact::Fact;
use crate::moe_protect::protection_action::ProtectionAction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolicRule {
    pub name: String,
    pub conditions: Vec<Fact>,
    pub conclusion_action: ProtectionAction,
    pub confidence: f64,
    pub reasoning: String,
}

impl SymbolicRule {
    pub fn new(
        name: impl Into<String>,
        conditions: Vec<Fact>,
        conclusion_action: ProtectionAction,
        confidence: f64,
        reasoning: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            conditions,
            conclusion_action,
            confidence,
            reasoning: reasoning.into(),
        }
    }
}
