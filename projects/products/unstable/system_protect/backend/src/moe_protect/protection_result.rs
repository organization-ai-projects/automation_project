use serde::{Deserialize, Serialize};

use super::expert_verdict::ExpertVerdict;
use super::protection_action::ProtectionAction;
use super::threat_id::ThreatId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectionResult {
    pub threat_id: ThreatId,
    pub verdicts: Vec<ExpertVerdict>,
    pub final_action: ProtectionAction,
    pub combined_confidence: f64,
    pub summary: String,
}
