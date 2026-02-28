use crate::events::SimEvent;
use crate::model::SimState;
use crate::time::Tick;

pub struct BuildEngine;

impl BuildEngine {
    pub fn tick(state: &mut SimState, tick: Tick, events: &mut Vec<SimEvent>) {
        for empire in state.empires.values_mut() {
            let empire_id = empire.empire_id;
            while let Some(order) = empire.build_queue.pop_if_done(tick) {
                if let Some(planet) = empire.planets.values_mut().next() {
                    let entry = planet
                        .building_levels
                        .entry(order.building_kind)
                        .or_insert(0);
                    *entry = order.target_level;
                    events.push(SimEvent::BuildCompleted {
                        tick,
                        empire_id,
                        planet_id: planet.planet_id,
                        building_kind: order.building_kind,
                        new_level: order.target_level,
                    });
                }
            }
        }
    }
}
