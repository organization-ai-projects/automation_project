use crate::analytics::analytics_event::AnalyticsEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnalyticsLog {
    pub events: Vec<AnalyticsEvent>,
}

impl AnalyticsLog {
    pub fn append(&mut self, event: AnalyticsEvent) {
        self.events.push(event);
    }
}
