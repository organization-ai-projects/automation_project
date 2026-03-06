use std::collections::BTreeMap;

use crate::economy::resource_kind::ResourceKind;
use crate::fleets::fleet::Fleet;
use crate::fleets::ship_kind::ShipKind;
use crate::model::empire_id::EmpireId;
use crate::model::fleet_id::FleetId;
use crate::model::sim_state::SimState;
use crate::tech::tech_kind::TechKind;

pub struct QueueEngine;

impl QueueEngine {
    /// Process one tick of all build and research queues.
    pub fn tick(state: &mut SimState) {
        let empire_ids: Vec<EmpireId> = state.empires.keys().cloned().collect();

        for empire_id in empire_ids {
            process_build_queue(state, &empire_id);
            process_research_queue(state, &empire_id);
        }
    }
}

fn process_build_queue(state: &mut SimState, empire_id: &EmpireId) {
    let item = state
        .build_queues
        .get(empire_id)
        .and_then(|queue| queue.items.first().cloned());
    let Some(item) = item else {
        return;
    };

    let (metal_cost, energy_cost, ship_kind) = match item.as_str() {
        "fighter" => (4, 2, ShipKind::Fighter),
        "cruiser" => (8, 4, ShipKind::Cruiser),
        "battleship" => (14, 7, ShipKind::Battleship),
        "transport" => (6, 3, ShipKind::Transport),
        "scout" => (3, 2, ShipKind::Scout),
        _ => (4, 2, ShipKind::Fighter),
    };

    let can_pay = if let Some(empire) = state.empires.get_mut(empire_id) {
        let enough_metal = empire.resources.get(ResourceKind::Metal) >= metal_cost;
        let enough_energy = empire.resources.get(ResourceKind::Energy) >= energy_cost;
        if enough_metal && enough_energy {
            empire.resources.spend(ResourceKind::Metal, metal_cost);
            empire.resources.spend(ResourceKind::Energy, energy_cost);
            true
        } else {
            false
        }
    } else {
        false
    };

    if !can_pay {
        return;
    }

    if let Some(queue) = state.build_queues.get_mut(empire_id) {
        queue.items.remove(0);
    }

    let home_system = match state.empires.get(empire_id) {
        Some(empire) => empire.home_system.clone(),
        None => return,
    };

    if let Some((_, fleet)) = state
        .fleets
        .iter_mut()
        .find(|(_, fleet)| fleet.empire_id == *empire_id && fleet.location == home_system)
    {
        *fleet.ships.entry(ship_kind).or_insert(0) += 1;
        return;
    }

    let mut ships: BTreeMap<ShipKind, u32> = BTreeMap::new();
    ships.insert(ship_kind, 1);
    let fleet_id = FleetId(format!(
        "fleet_{}_{}_{}",
        empire_id.0, state.current_turn.0, state.current_tick.0
    ));
    let fleet = Fleet {
        id: fleet_id.clone(),
        empire_id: empire_id.clone(),
        ships,
        location: home_system,
        destination: None,
    };
    state.fleets.insert(fleet_id, fleet);
}

fn process_research_queue(state: &mut SimState, empire_id: &EmpireId) {
    let item = state
        .research_queues
        .get(empire_id)
        .and_then(|queue| queue.items.first().cloned());
    let Some(item) = item else {
        return;
    };

    let can_pay = if let Some(empire) = state.empires.get_mut(empire_id) {
        empire.resources.spend(ResourceKind::Research, 3)
    } else {
        false
    };
    if !can_pay {
        return;
    }

    let tech_kind = match item.as_str() {
        "ship_drive" => TechKind::ShipDrive,
        "weapons" => TechKind::Weapons,
        "shields" => TechKind::Shields,
        "economics" => TechKind::Economics,
        "diplomacy" => TechKind::Diplomacy,
        _ => TechKind::Economics,
    };

    if let Some(empire) = state.empires.get_mut(empire_id) {
        empire.tech_tree.advance(tech_kind);
    }
    if let Some(queue) = state.research_queues.get_mut(empire_id) {
        queue.items.remove(0);
    }
}
