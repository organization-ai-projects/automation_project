// projects/products/unstable/digital_pet/backend/src/needs/needs_state.rs
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeedsState {
    pub hunger: u32,
    pub fatigue: u32,
    pub happiness: u32,
    pub discipline: u32,
    pub sick: bool,
    pub sick_ticks: u32,
}

impl Default for NeedsState {
    fn default() -> Self {
        Self {
            hunger: 0,
            fatigue: 0,
            happiness: 100,
            discipline: 100,
            sick: false,
            sick_ticks: 0,
        }
    }
}

impl NeedsState {
    pub fn decay(&mut self, tick: Tick) {
        let _ = tick;
        self.hunger = (self.hunger + 1).min(100);
        self.fatigue = (self.fatigue + 1).min(100);
        self.happiness = self.happiness.saturating_sub(1);
        self.discipline = self.discipline.saturating_sub(1);
        if self.hunger > 80 || self.fatigue > 80 {
            self.sick_ticks += 1;
            if self.sick_ticks > 5 {
                self.sick = true;
            }
        } else {
            self.sick_ticks = self.sick_ticks.saturating_sub(1);
            if self.sick_ticks == 0 {
                self.sick = false;
            }
        }
    }
    pub fn feed(&mut self) {
        self.hunger = self.hunger.saturating_sub(30);
    }
    pub fn rest(&mut self) {
        self.fatigue = self.fatigue.saturating_sub(30);
    }
    pub fn play(&mut self) {
        self.happiness = (self.happiness + 20).min(100);
    }
    pub fn discipline(&mut self) {
        self.discipline = (self.discipline + 20).min(100);
    }
    pub fn medicine(&mut self) {
        self.sick = false;
        self.sick_ticks = 0;
    }
}
