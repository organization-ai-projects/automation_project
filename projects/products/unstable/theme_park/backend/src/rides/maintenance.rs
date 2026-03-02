#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// Current maintenance state of a ride.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MaintenanceState {
    Operational,
    UnderMaintenance { ticks_remaining: u32 },
}

/// Maintenance record for a ride.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Maintenance {
    pub state: MaintenanceState,
    pub total_maintenance_events: u32,
}

impl Maintenance {
    pub fn new() -> Self {
        Self {
            state: MaintenanceState::Operational,
            total_maintenance_events: 0,
        }
    }

    pub fn is_operational(&self) -> bool {
        self.state == MaintenanceState::Operational
    }

    pub fn begin(&mut self, ticks: u32) {
        self.state = MaintenanceState::UnderMaintenance {
            ticks_remaining: ticks,
        };
        self.total_maintenance_events += 1;
    }

    /// Advance maintenance timer. Returns true when maintenance completes.
    pub fn advance_tick(&mut self) -> bool {
        if let MaintenanceState::UnderMaintenance {
            ref mut ticks_remaining,
        } = self.state
        {
            *ticks_remaining = ticks_remaining.saturating_sub(1);
            if *ticks_remaining == 0 {
                self.state = MaintenanceState::Operational;
                return true;
            }
        }
        false
    }
}

impl Default for Maintenance {
    fn default() -> Self {
        Self::new()
    }
}
