use serde::{Deserialize, Serialize};

use super::game_event::GameEvent;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventLog {
    pub events: Vec<GameEvent>,
}

impl EventLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, event: GameEvent) {
        self.events.push(event);
    }
}
