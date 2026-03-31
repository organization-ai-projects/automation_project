use super::expert::ProtectionExpert;
use super::expert_id::ExpertId;
use super::threat_event::ThreatEvent;

pub struct MoeRouter;

impl MoeRouter {
    pub fn route<'a>(
        event: &ThreatEvent,
        experts: &'a [Box<dyn ProtectionExpert>],
    ) -> Vec<&'a ExpertId> {
        experts
            .iter()
            .filter(|expert| expert.can_analyze(event))
            .map(|expert| expert.id())
            .collect()
    }
}
