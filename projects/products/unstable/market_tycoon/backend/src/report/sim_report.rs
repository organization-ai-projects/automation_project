use serde::{Deserialize, Serialize};

use crate::SimEngine;
use crate::config::sim_config::SimConfig;
use crate::events::event_log::EventLog;
use crate::report::company_report::CompanyReport;
use crate::report::run_hash::RunHash;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimReport {
    pub run_hash: String,
    pub seed: u64,
    pub ticks: u64,
    pub event_count: usize,
    pub net_profit: i64,
    pub companies: Vec<CompanyReport>,
}

impl SimReport {
    pub fn generate(engine: &SimEngine, event_log: &EventLog, config: &SimConfig) -> Self {
        let net_profit = engine.ledger.net_profit();
        let event_count = event_log.len();
        let run_hash = RunHash::compute(config.seed, config.ticks, event_count, net_profit);

        let companies = engine
            .companies
            .iter()
            .map(|(id, c)| {
                let store_count =
                    engine.stores.values().filter(|s| s.owner() == *id).count() as u64;
                CompanyReport {
                    company_id: *id,
                    name: c.name().to_string(),
                    final_budget: c.budget(),
                    store_count,
                }
            })
            .collect();

        Self {
            run_hash,
            seed: config.seed,
            ticks: config.ticks,
            event_count,
            net_profit,
            companies,
        }
    }
}
