use crate::build::BuildingKind;
use crate::diagnostics::SpaceEmpireError;
use crate::time::Tick;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildOrder {
    pub building_kind: BuildingKind,
    pub target_level: u32,
    pub started_at: Tick,
    pub finish_at: Tick,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildQueue {
    pub items: VecDeque<BuildOrder>,
}

impl BuildQueue {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, order: BuildOrder) -> Result<(), SpaceEmpireError> {
        if self.items.len() >= 5 {
            return Err(SpaceEmpireError::QueueFull(
                "Build queue is full (max 5)".to_string(),
            ));
        }
        self.items.push_back(order);
        Ok(())
    }

    pub fn peek(&self) -> Option<&BuildOrder> {
        self.items.front()
    }

    pub fn pop_if_done(&mut self, tick: Tick) -> Option<BuildOrder> {
        if self.items.front().is_some_and(|o| o.finish_at <= tick) {
            self.items.pop_front()
        } else {
            None
        }
    }
}

impl Default for BuildQueue {
    fn default() -> Self {
        Self::new()
    }
}
