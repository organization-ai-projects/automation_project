use crate::events::campaign_event::CampaignEvent;
use rand::Rng;
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDeck {
    pub cards: Vec<CampaignEvent>,
    pub drawn_indices: Vec<usize>,
}

impl EventDeck {
    pub fn new(cards: Vec<CampaignEvent>) -> Self {
        Self {
            cards,
            drawn_indices: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn draw(&mut self, rng: &mut StdRng) -> Option<CampaignEvent> {
        let available: Vec<usize> = (0..self.cards.len())
            .filter(|i| !self.drawn_indices.contains(i))
            .collect();
        if available.is_empty() {
            return None;
        }
        let pick = rng.random_range(0..available.len());
        let idx = available[pick];
        self.drawn_indices.push(idx);
        Some(self.cards[idx].clone())
    }

    #[allow(dead_code)]
    pub fn remaining(&self) -> usize {
        self.cards.len() - self.drawn_indices.len()
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.drawn_indices.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::candidate_id::CandidateId;
    use rand::SeedableRng;

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
    fn event_deck_determinism() {
        let mut deck1 = make_deck();
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut deck2 = make_deck();
        let mut rng2 = StdRng::seed_from_u64(42);

        let mut draws1 = Vec::new();
        let mut draws2 = Vec::new();
        for _ in 0..3 {
            deck1.draw(&mut rng1);
            draws1.push(deck1.drawn_indices.last().copied().unwrap_or(0));
            deck2.draw(&mut rng2);
            draws2.push(deck2.drawn_indices.last().copied().unwrap_or(0));
        }

        assert_eq!(draws1, draws2, "event deck draws must be deterministic");
    }
}
