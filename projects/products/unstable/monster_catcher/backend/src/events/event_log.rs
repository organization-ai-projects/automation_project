use crate::events::game_event::GameEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventLog {
    pub events: Vec<GameEvent>,
}

impl EventLog {
    pub fn push(&mut self, event: GameEvent) {
        self.events.push(event);
    }

    pub fn push_rng_draws(&mut self, draws: &[crate::rng::rng_draw::RngDraw]) {
        for draw in draws {
            self.events.push(GameEvent::RngDraw { draw: draw.clone() });
        }
    }
}
