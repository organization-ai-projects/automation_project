#![allow(dead_code)]
use crate::determinism::seed::Seed;
use crate::ecs::component::Component;
use crate::ecs::component_id::ComponentId;
use crate::ecs::world::World;
use crate::events::event_log::EventLog;
use crate::packs::pack::Pack;
use crate::packs::pack_id::PackId;
use crate::packs::pack_kind::PackKind;
use crate::time::logical_clock::LogicalClock;

const C_LABEL: ComponentId = ComponentId(0);
const C_COUNTER: ComponentId = ComponentId(1);

pub struct ThemeParkPack;

impl Pack for ThemeParkPack {
    fn id(&self) -> PackId { PackId::new("theme_park") }
    fn kind(&self) -> PackKind { PackKind::ThemePark }
    fn name(&self) -> &str { "Theme Park" }

    fn initialize(&self, world: &mut World, _seed: Seed) {
        let ride = world.spawn();
        world.insert_component(ride, C_LABEL, Component::Label("ride_roller_coaster".to_string()));
        world.insert_component(ride, C_COUNTER, Component::Counter(0));

        let visitors = world.spawn();
        world.insert_component(visitors, C_LABEL, Component::Label("visitor_queue".to_string()));
        world.insert_component(visitors, C_COUNTER, Component::Counter(10));
    }

    fn tick(&self, world: &mut World, clock: &LogicalClock, event_log: &mut EventLog) {
        let entities = world.entities_sorted();
        for eid in entities {
            let is_queue = matches!(world.get_component(eid, C_LABEL), Some(Component::Label(lbl)) if lbl == "visitor_queue");
            if is_queue {
                if let Some(Component::Counter(c)) = world.get_component_mut(eid, C_COUNTER) {
                    if *c > 0 {
                        *c -= 1;
                        event_log.emit(clock.tick, "theme_park.visitor_boarded", serde_json::json!({ "tick": clock.tick.0 }));
                    }
                }
            }
        }
    }
}
