use crate::moe_protect::error::ProtectError;
use crate::moe_protect::expert::ProtectionExpert;
use crate::moe_protect::expert_id::ExpertId;
use crate::moe_protect::expert_type::ExpertType;
use crate::moe_protect::expert_verdict::ExpertVerdict;
use crate::moe_protect::protection_action::ProtectionAction;
use crate::moe_protect::threat_event::ThreatEvent;
use crate::moe_protect::threat_type::ThreatType;

use super::rule_engine::RuleEngine;

pub struct FirewallExpert {
    id: ExpertId,
    rule_engine: RuleEngine,
}

impl FirewallExpert {
    pub fn new() -> Self {
        Self {
            id: ExpertId::new("firewall"),
            rule_engine: RuleEngine::with_defaults(),
        }
    }
}

impl ProtectionExpert for FirewallExpert {
    fn id(&self) -> &ExpertId {
        &self.id
    }

    fn expert_type(&self) -> ExpertType {
        ExpertType::Firewall
    }

    fn name(&self) -> &str {
        "Firewall Expert"
    }

    fn can_analyze(&self, event: &ThreatEvent) -> bool {
        matches!(
            event.threat_type,
            ThreatType::NetworkIntrusion
                | ThreatType::DenialOfService
                | ThreatType::UnauthorizedAccess
                | ThreatType::PortScan
                | ThreatType::BruteForce
                | ThreatType::DataExfiltration
        )
    }

    fn analyze(&self, event: &ThreatEvent) -> Result<ExpertVerdict, ProtectError> {
        if let Some(rule) = self.rule_engine.evaluate(&event.source, &event.target) {
            Ok(ExpertVerdict::new(
                self.id.clone(),
                rule.action.clone(),
                0.9,
                format!("Matched firewall rule: {}", rule.name),
            ))
        } else {
            // Default policy: allow with low confidence, suggest logging
            Ok(ExpertVerdict::new(
                self.id.clone(),
                ProtectionAction::Log,
                0.5,
                "No matching firewall rule, default to log",
            ))
        }
    }
}
