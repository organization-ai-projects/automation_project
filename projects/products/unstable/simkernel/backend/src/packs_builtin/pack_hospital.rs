use crate::determinism::seed::Seed;
use crate::ecs::component::Component;
use crate::ecs::component_id::ComponentId;
use crate::ecs::world::World;
use crate::events::event_log::EventLog;
use crate::packs::pack::Pack;
use crate::packs::pack_id::PackId;
use crate::packs::pack_kind::PackKind;
use crate::time::logical_clock::LogicalClock;
use common_json::{Json, JsonMap};

const C_LABEL: ComponentId = ComponentId(0);
const C_COUNTER: ComponentId = ComponentId(1);

pub struct PackHospital;

impl Pack for PackHospital {
    fn id(&self) -> PackId {
        PackId::new("hospital")
    }
    fn kind(&self) -> PackKind {
        PackKind::Hospital
    }
    fn name(&self) -> &str {
        "Hospital"
    }

    fn initialize(&self, world: &mut World, seed: Seed) {
        let seed_value = seed.value();
        let queue_start = (seed_value % 3) as i64;
        let room1 = world.spawn();
        world.insert_component(room1, C_LABEL, Component::Label("room_1".to_string()));
        world.insert_component(room1, C_COUNTER, Component::Counter(0));

        let room2 = world.spawn();
        world.insert_component(room2, C_LABEL, Component::Label("room_2".to_string()));
        world.insert_component(room2, C_COUNTER, Component::Counter(0));

        let queue = world.spawn();
        world.insert_component(
            queue,
            C_LABEL,
            Component::Label("patient_queue".to_string()),
        );
        world.insert_component(queue, C_COUNTER, Component::Counter(queue_start));
    }

    fn tick(&self, world: &mut World, clock: &LogicalClock, event_log: &mut EventLog) {
        let entities = world.entities_sorted();
        for eid in entities {
            let is_queue = matches!(world.get_component(eid, C_LABEL), Some(Component::Label(lbl)) if lbl == "patient_queue");
            if is_queue && let Some(Component::Counter(c)) = world.get_component_mut(eid, C_COUNTER)
            {
                *c += 1;
                let mut payload = JsonMap::new();
                payload.insert("tick".to_string(), Json::from(clock.tick.0));
                payload.insert("queue_size".to_string(), Json::from(*c));
                event_log.emit(
                    clock.tick,
                    "hospital.patient_arrived",
                    Json::Object(payload),
                );
            }
        }
    }
}
