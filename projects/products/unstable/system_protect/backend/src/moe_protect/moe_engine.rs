use super::engine_status::EngineStatus;
use super::error::ProtectError;
use super::expert::ProtectionExpert;
use super::expert_info::ExpertInfo;
use super::moe_aggregator::MoeAggregator;
use super::moe_router::MoeRouter;
use super::protection_result::ProtectionResult;
use super::threat_event::ThreatEvent;
use crate::antivirus::antivirus_expert::AntivirusExpert;
use crate::antivirus::signature::Signature;
use crate::firewall::firewall_expert::FirewallExpert;
use crate::firewall::firewall_rule::FirewallRule;
use crate::symbolic_engine::symbolic_expert::SymbolicExpert;

pub struct MoeEngine {
    experts: Vec<Box<dyn ProtectionExpert>>,
    events_analyzed: u64,
    threats_blocked: u64,
    firewall_rules: Vec<FirewallRule>,
    signatures: Vec<Signature>,
}

impl MoeEngine {
    pub fn new() -> Self {
        Self {
            experts: Vec::new(),
            events_analyzed: 0,
            threats_blocked: 0,
            firewall_rules: Vec::new(),
            signatures: Vec::new(),
        }
    }

    pub fn register_default_experts(&mut self) {
        self.experts.push(Box::new(AntivirusExpert::new()));
        self.experts.push(Box::new(FirewallExpert::new()));
        self.experts.push(Box::new(SymbolicExpert::new()));
    }

    pub fn add_firewall_rule(&mut self, rule: FirewallRule) {
        self.firewall_rules.push(rule.clone());
        // Update the firewall expert if present
        for expert in &mut self.experts {
            if expert.expert_type() == crate::moe_protect::expert_type::ExpertType::Firewall {
                // Rebuild the firewall expert with updated rules
                // This is a simplified approach for the MVP
            }
        }
    }

    pub fn add_signature(&mut self, signature: Signature) {
        self.signatures.push(signature);
    }

    pub fn analyze(&mut self, event: ThreatEvent) -> Result<ProtectionResult, ProtectError> {
        let routed_ids = MoeRouter::route(&event, &self.experts);

        if routed_ids.is_empty() {
            return Err(ProtectError::NoExpertAvailable(format!(
                "No expert can analyze threat type {:?}",
                event.threat_type
            )));
        }

        let mut verdicts = Vec::new();
        for expert in &self.experts {
            if routed_ids.contains(&expert.id()) {
                match expert.analyze(&event) {
                    Ok(verdict) => verdicts.push(verdict),
                    Err(e) => {
                        eprintln!("Expert {} failed: {e}", expert.id());
                    }
                }
            }
        }

        if verdicts.is_empty() {
            return Err(ProtectError::AnalysisFailed(
                "All experts failed to analyze threat".to_string(),
            ));
        }

        let (final_action, combined_confidence, summary) = MoeAggregator::aggregate(&verdicts);

        self.events_analyzed += 1;
        if final_action == crate::moe_protect::protection_action::ProtectionAction::Block
            || final_action == crate::moe_protect::protection_action::ProtectionAction::Quarantine
        {
            self.threats_blocked += 1;
        }

        Ok(ProtectionResult {
            threat_id: event.id,
            verdicts,
            final_action,
            combined_confidence,
            summary,
        })
    }

    pub fn list_experts(&self) -> Vec<ExpertInfo> {
        self.experts
            .iter()
            .map(|e| ExpertInfo {
                id: e.id().clone(),
                name: e.name().to_string(),
                expert_type: e.expert_type(),
            })
            .collect()
    }

    pub fn status(&self) -> EngineStatus {
        EngineStatus {
            expert_count: self.experts.len(),
            firewall_rule_count: self.firewall_rules.len(),
            signature_count: self.signatures.len(),
            events_analyzed: self.events_analyzed,
            threats_blocked: self.threats_blocked,
        }
    }
}
