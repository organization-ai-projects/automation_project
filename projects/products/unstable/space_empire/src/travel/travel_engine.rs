use crate::events::SimEvent;
use crate::model::{FleetId, SimState};
use crate::ships::base_stats;
use crate::time::Tick;
use crate::travel::{Route, TravelOrder};
use std::collections::BTreeMap;

pub struct TravelEngine {
    pub orders: BTreeMap<FleetId, TravelOrder>,
}

#[allow(dead_code)]
impl TravelEngine {
    pub fn new() -> Self {
        Self {
            orders: BTreeMap::new(),
        }
    }

    pub fn dispatch(&mut self, order: TravelOrder) {
        self.orders.insert(order.fleet_id, order);
    }

    pub fn compute_arrival(route: &Route, departure: Tick, fleet: &crate::ships::Fleet) -> Tick {
        let base_speed: u64 = fleet
            .ships
            .iter()
            .filter(|&(_, &count)| count > 0)
            .map(|(&kind, _)| base_stats(kind).speed)
            .min()
            .unwrap_or(1000);
        let distance = route.travel_distance();
        let travel_ticks = (distance / base_speed).max(1);
        Tick(departure.0 + travel_ticks)
    }

    pub fn tick(&mut self, state: &mut SimState, tick: Tick, events: &mut Vec<SimEvent>) {
        let arrived: Vec<FleetId> = self
            .orders
            .iter()
            .filter(|(_, order)| order.arrival <= tick)
            .map(|(id, _)| *id)
            .collect();

        for fleet_id in arrived {
            if let Some(order) = self.orders.remove(&fleet_id) {
                for empire in state.empires.values_mut() {
                    if let Some(fleet) = empire.fleets.get_mut(&fleet_id) {
                        fleet.location = order.route.to;
                        events.push(SimEvent::FleetArrived {
                            tick,
                            fleet_id,
                            location: order.route.to,
                        });
                        break;
                    }
                }
            }
        }
    }
}

impl Default for TravelEngine {
    fn default() -> Self {
        Self::new()
    }
}
