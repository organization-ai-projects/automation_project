use crate::events::campaign_event::CampaignEvent;
use deterministic_rng::Rng;
use deterministic_rng::rngs::StdRng;
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

    pub fn draw(&mut self, rng: &mut StdRng) -> Option<(usize, CampaignEvent)> {
        let available: Vec<usize> = (0..self.cards.len())
            .filter(|i| !self.drawn_indices.contains(i))
            .collect();
        if available.is_empty() {
            return None;
        }
        let pick = rng.random_range(0..available.len());
        let idx = available[pick];
        self.drawn_indices.push(idx);
        Some((idx, self.cards[idx].clone()))
    }
}
