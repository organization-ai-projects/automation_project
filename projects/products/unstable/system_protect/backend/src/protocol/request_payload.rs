use serde::{Deserialize, Serialize};

use crate::antivirus::signature::Signature;
use crate::firewall::firewall_rule::FirewallRule;
use crate::moe_protect::threat_event::ThreatEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RequestPayload {
    AnalyzeThreat { threat_event: ThreatEvent },
    ListExperts,
    GetStatus,
    AddFirewallRule { rule: FirewallRule },
    AddSignature { signature: Signature },
    Shutdown,
}
