// projects/products/unstable/hospital_tycoon/backend/src/replay/replay_engine.rs
use crate::config::sim_config::SimConfig;
use crate::model::hospital_state::HospitalState;
use crate::replay::replay_file::ReplayFile;
use crate::report::run_report::RunReport;
use crate::sim::event_log::EventLog;
use crate::sim::sim_engine::SimEngine;
use crate::time::tick_clock::TickClock;

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn run(
        replay: &ReplayFile,
        config: &SimConfig,
    ) -> (HospitalState, TickClock, EventLog, RunReport) {
        let mut engine = SimEngine::new(replay.seed, replay.ticks, config.clone());
        while !engine.clock.is_done() {
            engine.step_one();
        }
        let report = RunReport::generate(
            &engine.state,
            &engine.clock,
            &engine.event_log,
            &replay.scenario_name,
        );
        (engine.state, engine.clock, engine.event_log, report)
    }
}
