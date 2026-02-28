use crate::events::SimEvent;

pub struct EventLog {
    pub events: Vec<SimEvent>,
}

#[allow(dead_code)]
impl EventLog {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push(&mut self, event: SimEvent) {
        self.events.push(event);
    }

    pub fn iter(&self) -> impl Iterator<Item = &SimEvent> {
        self.events.iter()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl Default for EventLog {
    fn default() -> Self {
        Self::new()
    }
}
