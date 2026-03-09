use crate::events::campaign_event::CampaignEvent;
use crate::model::candidate_id::CandidateId;

#[test]
fn campaign_event_exposes_target_and_delta() {
    let event = CampaignEvent::Scandal {
        target: CandidateId::new("alice"),
        description: "bad day".to_string(),
        severity: 3,
        approval_delta: -0.2,
    };

    let maybe_target = event.target_candidate();
    assert!(maybe_target.is_some());
    if let Some(target) = maybe_target {
        assert_eq!(target.to_string(), "alice");
    }
    assert_eq!(event.approval_delta(), -0.2);
}
