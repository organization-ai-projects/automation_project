use serde::{Deserialize, Serialize};

use super::expert_id::ExpertId;
use super::protection_action::ProtectionAction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertVerdict {
    pub expert_id: ExpertId,
    pub action: ProtectionAction,
    pub confidence: f64,
    pub reasoning: String,
}

impl ExpertVerdict {
    pub fn new(
        expert_id: ExpertId,
        action: ProtectionAction,
        confidence: f64,
        reasoning: impl Into<String>,
    ) -> Self {
        Self {
            expert_id,
            action,
            confidence,
            reasoning: reasoning.into(),
        }
    }
}
