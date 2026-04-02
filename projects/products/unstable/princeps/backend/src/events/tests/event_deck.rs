use crate::events::campaign_event::CampaignEvent;
use crate::events::event_deck::EventDeck;
use crate::model::candidate_id::CandidateId;
use deterministic_rng::{SeedableRng, rngs::StdRng};

fn make_deck() -> EventDeck {
    EventDeck::new(vec![
        CampaignEvent::Gaffe {
            target: CandidateId::new("a"),
            description: "oops".to_string(),
            approval_delta: -0.05,
        },
        CampaignEvent::Endorsement {
            target: CandidateId::new("b"),
            source: "org".to_string(),
            approval_delta: 0.05,
        },
        CampaignEvent::PolicyWin {
            target: CandidateId::new("a"),
            topic: "economy".to_string(),
            approval_delta: 0.04,
        },
    ])
}

#[test]
fn event_deck_draw_records_drawn_index() {
    let mut deck = make_deck();
    let mut rng = StdRng::seed_from_u64(5);

    let draw = deck.draw(&mut rng);
    assert!(draw.is_some());
    assert_eq!(deck.drawn_indices.len(), 1);
}
