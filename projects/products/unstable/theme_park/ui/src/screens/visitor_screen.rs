#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// View of visitor states.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisitorScreen {
    pub visitor_id: u32,
    pub mood: i32,
    pub patience: u32,
    pub rides_completed: u32,
    pub status: String,
}

impl VisitorScreen {
    pub fn render(&self) -> String {
        format!(
            "[Visitor {}] mood={} patience={} rides={} status={}",
            self.visitor_id, self.mood, self.patience, self.rides_completed, self.status
        )
    }
}
