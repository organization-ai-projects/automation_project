// projects/products/unstable/digital_pet/backend/src/replay/replay_engine.rs
use crate::care::care_engine::CareEngine;
use crate::events::event_log::EventLog;
use crate::evolution::evolution_engine::EvolutionEngine;
use crate::evolution::evolution_tree::EvolutionTree;
use crate::model::pet::Pet;
use crate::needs::needs_state::NeedsState;
use crate::replay::replay_file::ReplayFile;
use crate::time::tick_clock::TickClock;

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn run(replay: &ReplayFile) -> (Pet, NeedsState, TickClock, EventLog, CareEngine) {
        let mut clock = TickClock::new(replay.seed, replay.ticks);
        let mut pet = Pet::new(replay.seed, replay.scenario.starting_species.clone());
        let mut needs = NeedsState::default();
        let mut event_log = EventLog::new();
        let mut care_engine = CareEngine::new();
        let tree = EvolutionTree::from_config(&replay.scenario.config);
        let mut evolution_engine = EvolutionEngine::new(tree);

        while !clock.is_done() {
            clock.tick();
            let tick = clock.current_tick();
            for action in &replay.actions {
                if action.tick == tick {
                    care_engine.apply_action(action.kind.clone(), &mut needs, tick);
                }
            }
            needs.decay(tick);
            care_engine.evaluate(&needs, tick);
            evolution_engine.evaluate(&mut pet, &needs, &care_engine, tick, &mut event_log);
        }
        (pet, needs, clock, event_log, care_engine)
    }
}
