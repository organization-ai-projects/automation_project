use crate::moe_protect::error::ProtectError;
use crate::moe_protect::expert::ProtectionExpert;
use crate::moe_protect::expert_id::ExpertId;
use crate::moe_protect::expert_type::ExpertType;
use crate::moe_protect::expert_verdict::ExpertVerdict;
use crate::moe_protect::protection_action::ProtectionAction;
use crate::moe_protect::threat_event::ThreatEvent;

use super::inference_engine::InferenceEngine;

pub struct SymbolicExpert {
    id: ExpertId,
    inference_engine: InferenceEngine,
}

impl SymbolicExpert {
    pub fn new() -> Self {
        Self {
            id: ExpertId::new("symbolic"),
            inference_engine: InferenceEngine::with_defaults(),
        }
    }
}

impl ProtectionExpert for SymbolicExpert {
    fn id(&self) -> &ExpertId {
        &self.id
    }

    fn expert_type(&self) -> ExpertType {
        ExpertType::SymbolicAnalyzer
    }

    fn name(&self) -> &str {
        "Symbolic Analyzer"
    }

    fn can_analyze(&self, _event: &ThreatEvent) -> bool {
        // The symbolic engine can analyze all threat types
        true
    }

    fn analyze(&self, event: &ThreatEvent) -> Result<ExpertVerdict, ProtectError> {
        match self.inference_engine.infer(event) {
            Some((action, confidence, reasoning)) => Ok(ExpertVerdict::new(
                self.id.clone(),
                action,
                confidence,
                reasoning,
            )),
            None => Ok(ExpertVerdict::new(
                self.id.clone(),
                ProtectionAction::Log,
                0.3,
                "No symbolic rules matched, defaulting to log",
            )),
        }
    }
}
