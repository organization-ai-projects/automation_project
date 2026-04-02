use super::error::ProtectError;
use super::expert_id::ExpertId;
use super::expert_type::ExpertType;
use super::expert_verdict::ExpertVerdict;
use super::threat_event::ThreatEvent;

pub trait ProtectionExpert: Send + Sync {
    fn id(&self) -> &ExpertId;
    fn expert_type(&self) -> ExpertType;
    fn name(&self) -> &str;
    fn can_analyze(&self, event: &ThreatEvent) -> bool;
    fn analyze(&self, event: &ThreatEvent) -> Result<ExpertVerdict, ProtectError>;
}
