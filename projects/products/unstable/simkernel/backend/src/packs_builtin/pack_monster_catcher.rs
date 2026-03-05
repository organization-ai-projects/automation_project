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

pub struct PackMonsterCatcher;

impl Pack for PackMonsterCatcher {
    fn id(&self) -> PackId {
        PackId::new("monster_catcher")
    }
    fn kind(&self) -> PackKind {
        PackKind::MonsterCatcher
    }
    fn name(&self) -> &str {
        "Monster Catcher"
    }

    fn initialize(&self, world: &mut World, seed: Seed) {
        let player = world.spawn();
        world.insert_component(player, C_LABEL, Component::Label("player".to_string()));
        world.insert_component(
            player,
            C_COUNTER,
            Component::Counter(seed.value() as i64 % 100),
        );

        let monster = world.spawn();
        world.insert_component(
            monster,
            C_LABEL,
            Component::Label("monster_001".to_string()),
        );
        world.insert_component(monster, C_COUNTER, Component::Counter(50));
    }

    fn tick(&self, world: &mut World, clock: &LogicalClock, event_log: &mut EventLog) {
        let roll = clock.tick.0.wrapping_mul(37).wrapping_add(13) % 100;

        if roll < 20 {
            let entities = world.entities_sorted();
            for eid in entities {
                let is_monster = matches!(world.get_component(eid, C_LABEL), Some(Component::Label(lbl)) if lbl == "monster_001");
                if is_monster {
                    event_log.emit(clock.tick, "monster_catcher.encounter", {
                        let mut payload = JsonMap::new();
                        payload.insert("tick".to_string(), Json::from(clock.tick.0));
                        payload.insert("roll".to_string(), Json::from(roll));
                        Json::Object(payload)
                    });
                    if let Some(Component::Counter(hp)) = world.get_component_mut(eid, C_COUNTER) {
                        *hp = hp.saturating_sub(10);
                    }
                }
            }
        }
    }
}
