use crate::events::colony_event::ColonyEvent;
use crate::rng::rng_draw::RngDraw;
use rand::RngCore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDeck {
    pub events: Vec<ColonyEvent>,
}

impl EventDeck {
    pub fn default_deck() -> Self {
        Self {
            events: vec![
                ColonyEvent::Raid { severity: 2 },
                ColonyEvent::Sickness { colonist_name: "unknown".to_string() },
                ColonyEvent::Traders { goods: vec!["food".to_string(), "tools".to_string()] },
                ColonyEvent::Windfall { resource: "stone".to_string(), amount: 10 },
            ],
        }
    }
    pub fn draw(&self, rng: &mut impl RngCore, draw_log: &mut Vec<RngDraw>) -> Option<(usize, &ColonyEvent)> {
        if self.events.is_empty() { return None; }
        let raw = rng.next_u64();
        let idx = (raw % self.events.len() as u64) as usize;
        draw_log.push(RngDraw { raw_value: raw, resolved_index: idx });
        Some((idx, &self.events[idx]))
    }
}
