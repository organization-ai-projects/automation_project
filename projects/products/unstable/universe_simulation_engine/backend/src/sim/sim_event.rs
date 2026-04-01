use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimEvent {
    EraTransition { era_name: String },
    ParticlesCreated { count: usize },
    ParticlesCombined { count: usize },
    StarFormed { mass: f64 },
    StarDied { class: String },
    GalaxyFormed { galaxy_type: String },
    FilamentFormed,
    UniverseExpanded { scale_factor: f64 },
    PhysicsApplied { engine: String },
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventLog {
    pub entries: Vec<(Tick, SimEvent)>,
}

impl EventLog {
    pub fn record(&mut self, tick: Tick, event: SimEvent) {
        self.entries.push((tick, event));
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
