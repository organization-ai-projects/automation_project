use crate::events::SimEvent;
use crate::model::SimState;
use crate::time::Tick;

pub struct ResearchEngine;

impl ResearchEngine {
    pub fn tick(state: &mut SimState, tick: Tick, events: &mut Vec<SimEvent>) {
        for empire in state.empires.values_mut() {
            let empire_id = empire.empire_id;
            while let Some(order) = empire.research_queue.pop_if_done(tick) {
                empire.researched_techs.insert(order.tech_kind);
                events.push(SimEvent::ResearchCompleted {
                    tick,
                    empire_id,
                    tech_kind: order.tech_kind,
                    new_level: order.target_level,
                });
            }
        }
    }
}
