use crate::build::BuildingKind;
use crate::combat::BattleReport;
use crate::economy::ResourceKind;
use crate::model::{EmpireId, FleetId, PlanetId};
use crate::research::TechKind;
use crate::time::Tick;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimEvent {
    ResourcesProduced {
        tick: Tick,
        empire_id: EmpireId,
        amounts: BTreeMap<ResourceKind, u64>,
    },
    BuildCompleted {
        tick: Tick,
        empire_id: EmpireId,
        planet_id: PlanetId,
        building_kind: BuildingKind,
        new_level: u32,
    },
    ResearchCompleted {
        tick: Tick,
        empire_id: EmpireId,
        tech_kind: TechKind,
        new_level: u32,
    },
    FleetArrived {
        tick: Tick,
        fleet_id: FleetId,
        location: PlanetId,
    },
    CombatResolved {
        tick: Tick,
        report: BattleReport,
    },
}

impl SimEvent {
    pub fn tick(&self) -> Tick {
        match self {
            SimEvent::ResourcesProduced { tick, .. } => *tick,
            SimEvent::BuildCompleted { tick, .. } => *tick,
            SimEvent::ResearchCompleted { tick, .. } => *tick,
            SimEvent::FleetArrived { tick, .. } => *tick,
            SimEvent::CombatResolved { tick, .. } => *tick,
        }
    }
}
