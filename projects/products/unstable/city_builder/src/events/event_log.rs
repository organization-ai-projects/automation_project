use super::SimEvent;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventLog {
    pub events: Vec<SimEvent>,
}

impl EventLog {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push(&mut self, event: SimEvent) {
        self.events.push(event);
    }
}

impl Default for EventLog {
    fn default() -> Self {
        Self::new()
    }
}
