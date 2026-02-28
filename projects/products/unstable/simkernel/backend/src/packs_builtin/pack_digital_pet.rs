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
const C_FLAG: ComponentId = ComponentId(2);

pub struct DigitalPetPack;

impl Pack for DigitalPetPack {
    fn id(&self) -> PackId {
        PackId::new("digital_pet")
    }
    fn kind(&self) -> PackKind {
        PackKind::DigitalPet
    }
    fn name(&self) -> &str {
        "Digital Pet"
    }

    fn initialize(&self, world: &mut World, _seed: Seed) {
        let pet = world.spawn();
        world.insert_component(pet, C_LABEL, Component::Label("pet".to_string()));
        world.insert_component(pet, C_COUNTER, Component::Counter(100));
        world.insert_component(pet, C_FLAG, Component::Flag(true));
    }

    fn tick(&self, world: &mut World, clock: &LogicalClock, event_log: &mut EventLog) {
        let entities = world.entities_sorted();
        for eid in entities {
            let is_pet = matches!(world.get_component(eid, C_LABEL), Some(Component::Label(lbl)) if lbl == "pet");
            if is_pet
                && let Some(Component::Counter(happiness)) = world.get_component_mut(eid, C_COUNTER)
            {
                *happiness = happiness.saturating_sub(1);
                if *happiness % 10 == 0 {
                    event_log.emit(
                        clock.tick,
                        "digital_pet.needs_care",
                        serde_json::json!({
                            "tick": clock.tick.0, "happiness": *happiness
                        }),
                    );
                }
            }
        }
    }
}
