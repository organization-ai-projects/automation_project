use crate::moe_protect::error::ProtectError;
use crate::moe_protect::expert::ProtectionExpert;
use crate::moe_protect::expert_id::ExpertId;
use crate::moe_protect::expert_type::ExpertType;
use crate::moe_protect::expert_verdict::ExpertVerdict;
use crate::moe_protect::protection_action::ProtectionAction;
use crate::moe_protect::threat_event::ThreatEvent;
use crate::moe_protect::threat_type::ThreatType;

use super::signature_db::SignatureDb;

pub struct AntivirusExpert {
    id: ExpertId,
    signature_db: SignatureDb,
}

impl AntivirusExpert {
    pub fn new() -> Self {
        Self {
            id: ExpertId::new("antivirus"),
            signature_db: SignatureDb::with_defaults(),
        }
    }
}

impl ProtectionExpert for AntivirusExpert {
    fn id(&self) -> &ExpertId {
        &self.id
    }

    fn expert_type(&self) -> ExpertType {
        ExpertType::Antivirus
    }

    fn name(&self) -> &str {
        "Antivirus Expert"
    }

    fn can_analyze(&self, event: &ThreatEvent) -> bool {
        matches!(
            event.threat_type,
            ThreatType::Virus
                | ThreatType::Malware
                | ThreatType::Trojan
                | ThreatType::Ransomware
                | ThreatType::Worm
        )
    }

    fn analyze(&self, event: &ThreatEvent) -> Result<ExpertVerdict, ProtectError> {
        let matches = self.signature_db.scan(&event.payload);

        if matches.is_empty() {
            Ok(ExpertVerdict::new(
                self.id.clone(),
                ProtectionAction::Allow,
                0.3,
                "No known signatures matched",
            ))
        } else {
            let sig_names: Vec<&str> = matches.iter().map(|s| s.name.as_str()).collect();
            let max_severity = matches
                .iter()
                .map(|s| match s.severity.as_str() {
                    "critical" => 4,
                    "high" => 3,
                    "medium" => 2,
                    "low" => 1,
                    _ => 0,
                })
                .max()
                .unwrap_or(0);

            let (action, confidence) = match max_severity {
                4 => (ProtectionAction::Quarantine, 0.99),
                3 => (ProtectionAction::Block, 0.95),
                2 => (ProtectionAction::Alert, 0.8),
                _ => (ProtectionAction::Log, 0.6),
            };

            Ok(ExpertVerdict::new(
                self.id.clone(),
                action,
                confidence,
                format!("Matched signatures: {}", sig_names.join(", ")),
            ))
        }
    }
}
