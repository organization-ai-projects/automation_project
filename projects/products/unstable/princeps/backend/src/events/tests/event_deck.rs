use crate::events::campaign_event::CampaignEvent;
use crate::events::event_deck::EventDeck;
use crate::model::candidate_id::CandidateId;

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
fn event_deck_reset_restores_remaining_cards() {
    let mut deck = make_deck();
    deck.drawn_indices.push(0);
    deck.drawn_indices.push(1);
    assert_eq!(deck.remaining(), 1);

    deck.reset();
    assert_eq!(deck.remaining(), 3);
}
