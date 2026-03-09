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
