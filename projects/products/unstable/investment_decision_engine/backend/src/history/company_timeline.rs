use serde::{Deserialize, Serialize};

use crate::history::CompanyEvent;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompanyTimeline {
    pub ticker: String,
    pub events: Vec<CompanyEvent>,
}

impl CompanyTimeline {
    pub fn new(ticker: impl Into<String>) -> Self {
        Self {
            ticker: ticker.into(),
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: CompanyEvent) {
        self.events.push(event);
    }

    pub fn events_in_range(&self, start: &str, end: &str) -> Vec<&CompanyEvent> {
        self.events
            .iter()
            .filter(|e| e.date.as_str() >= start && e.date.as_str() <= end)
            .collect()
    }
}
