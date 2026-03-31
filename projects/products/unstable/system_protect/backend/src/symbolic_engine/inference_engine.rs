use super::fact::Fact;
use super::rule::SymbolicRule;
use crate::moe_protect::protection_action::ProtectionAction;
use crate::moe_protect::threat_event::ThreatEvent;
use crate::moe_protect::threat_level::ThreatLevel;
use crate::moe_protect::threat_type::ThreatType;

pub struct InferenceEngine {
    rules: Vec<SymbolicRule>,
}

impl InferenceEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn with_defaults() -> Self {
        let mut engine = Self::new();

        // Rule: High severity virus → Quarantine
        engine.add_rule(SymbolicRule::new(
            "high-severity-virus-quarantine",
            vec![
                Fact::new("threat", "is_type", "virus_family"),
                Fact::new("threat", "has_level", "high_or_critical"),
            ],
            ProtectionAction::Quarantine,
            0.95,
            "High severity virus family threats should be quarantined",
        ));

        // Rule: Repeated access attempts → Block
        engine.add_rule(SymbolicRule::new(
            "brute-force-block",
            vec![
                Fact::new("threat", "is_type", "access_attempt"),
                Fact::new("threat", "has_level", "medium_or_above"),
            ],
            ProtectionAction::Block,
            0.9,
            "Repeated unauthorized access attempts should be blocked",
        ));

        // Rule: Network scan → Alert
        engine.add_rule(SymbolicRule::new(
            "network-scan-alert",
            vec![
                Fact::new("threat", "is_type", "network_probe"),
                Fact::new("threat", "has_level", "any"),
            ],
            ProtectionAction::Alert,
            0.7,
            "Network scanning activity should trigger an alert",
        ));

        // Rule: Low severity → Log
        engine.add_rule(SymbolicRule::new(
            "low-severity-log",
            vec![Fact::new("threat", "has_level", "low_or_info")],
            ProtectionAction::Log,
            0.5,
            "Low severity events should be logged for review",
        ));

        engine
    }

    pub fn add_rule(&mut self, rule: SymbolicRule) {
        self.rules.push(rule);
    }

    pub fn derive_facts(event: &ThreatEvent) -> Vec<Fact> {
        let mut facts = Vec::new();

        // Derive type-category facts
        let type_category = match &event.threat_type {
            ThreatType::Virus
            | ThreatType::Malware
            | ThreatType::Trojan
            | ThreatType::Ransomware
            | ThreatType::Worm => "virus_family",
            ThreatType::NetworkIntrusion
            | ThreatType::DenialOfService
            | ThreatType::DataExfiltration => "network_probe",
            ThreatType::UnauthorizedAccess | ThreatType::BruteForce => "access_attempt",
            ThreatType::PortScan => "network_probe",
            ThreatType::Custom(_) => "unknown",
        };
        facts.push(Fact::new("threat", "is_type", type_category));

        // Derive level facts
        let level_category = match &event.threat_level {
            ThreatLevel::Info | ThreatLevel::Low => "low_or_info",
            ThreatLevel::Medium => "medium_or_above",
            ThreatLevel::High | ThreatLevel::Critical => "high_or_critical",
        };
        facts.push(Fact::new("threat", "has_level", level_category));
        // "any" level always matches
        facts.push(Fact::new("threat", "has_level", "any"));
        // medium_or_above also includes high_or_critical
        if matches!(
            event.threat_level,
            ThreatLevel::High | ThreatLevel::Critical
        ) {
            facts.push(Fact::new("threat", "has_level", "medium_or_above"));
        }

        facts
    }

    pub fn infer(&self, event: &ThreatEvent) -> Option<(ProtectionAction, f64, String)> {
        let facts = Self::derive_facts(event);

        let mut best_match: Option<&SymbolicRule> = None;

        for rule in &self.rules {
            let all_conditions_met = rule.conditions.iter().all(|condition| {
                facts.iter().any(|fact| {
                    fact.subject == condition.subject
                        && fact.predicate == condition.predicate
                        && fact.object == condition.object
                })
            });

            if all_conditions_met {
                if best_match.map_or(true, |best| rule.confidence > best.confidence) {
                    best_match = Some(rule);
                }
            }
        }

        best_match.map(|rule| {
            (
                rule.conclusion_action.clone(),
                rule.confidence,
                format!("Rule '{}': {}", rule.name, rule.reasoning),
            )
        })
    }
}
