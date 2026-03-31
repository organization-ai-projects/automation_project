use crate::config::sim_config::{CompanyConfig, SimConfig, StoreConfig};
use crate::demand::demand_model::DemandModel;
use crate::diagnostics::error::Error;
use crate::pricing::pricing_policy::PricingPolicy;
use crate::replay::replay_file::ReplayFile;
use crate::report::sim_report::SimReport;
use crate::snapshot::state_snapshot::StateSnapshot;

pub struct ReplayEngine;

impl ReplayEngine {
    pub fn replay(replay: &ReplayFile) -> Result<SimReport, Error> {
        let config = Self::minimal_config(replay.seed, replay.ticks);
        let mut engine = crate::create_engine(&config);
        let event_log = crate::run_simulation(&mut engine, &config);
        Ok(SimReport::generate(&engine, &event_log, &config))
    }

    pub fn snapshot_at(replay: &ReplayFile, at_tick: u64) -> Result<StateSnapshot, Error> {
        let config = Self::minimal_config(replay.seed, at_tick);
        let mut engine = crate::create_engine(&config);
        let event_log = crate::run_simulation(&mut engine, &config);
        Ok(StateSnapshot::capture(&engine, &event_log))
    }

    fn minimal_config(seed: u64, ticks: u64) -> SimConfig {
        SimConfig {
            seed,
            ticks,
            companies: vec![CompanyConfig {
                name: "ReplayCompany".into(),
                budget: 100000,
                stores: vec![StoreConfig {
                    name: "ReplayStore".into(),
                }],
            }],
            contracts: vec![],
            pricing_policy: PricingPolicy::default(),
            demand_model: DemandModel::default(),
            segments: vec![],
        }
    }
}
