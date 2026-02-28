use crate::diagnostics::SpaceEmpireError;
use crate::research::TechKind;
use crate::time::Tick;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchOrder {
    pub tech_kind: TechKind,
    pub target_level: u32,
    pub started_at: Tick,
    pub finish_at: Tick,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchQueue {
    pub items: VecDeque<ResearchOrder>,
}

impl ResearchQueue {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, order: ResearchOrder) -> Result<(), SpaceEmpireError> {
        if self.items.len() >= 2 {
            return Err(SpaceEmpireError::QueueFull(
                "Research queue is full (max 1 active + 1 queued)".to_string(),
            ));
        }
        self.items.push_back(order);
        Ok(())
    }

    pub fn peek(&self) -> Option<&ResearchOrder> {
        self.items.front()
    }

    pub fn pop_if_done(&mut self, tick: Tick) -> Option<ResearchOrder> {
        if self.items.front().is_some_and(|o| o.finish_at <= tick) {
            self.items.pop_front()
        } else {
            None
        }
    }
}

impl Default for ResearchQueue {
    fn default() -> Self {
        Self::new()
    }
}
