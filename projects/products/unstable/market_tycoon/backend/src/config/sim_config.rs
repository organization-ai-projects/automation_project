use serde::{Deserialize, Serialize};

use crate::demand::customer_segment::CustomerSegment;
use crate::demand::demand_model::DemandModel;
use crate::pricing::pricing_policy::PricingPolicy;
use crate::scenario::scenario::Scenario;
use crate::supply::contract::Contract;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimConfig {
    pub seed: u64,
    pub ticks: u64,
    pub companies: Vec<CompanyConfig>,
    pub contracts: Vec<Contract>,
    pub pricing_policy: PricingPolicy,
    pub demand_model: DemandModel,
    pub segments: Vec<CustomerSegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyConfig {
    pub name: String,
    pub budget: i64,
    pub stores: Vec<StoreConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreConfig {
    pub name: String,
}

impl SimConfig {
    pub fn from_scenario(scenario: &Scenario, seed: u64, ticks: u64) -> Self {
        Self {
            seed,
            ticks,
            companies: scenario.companies.clone(),
            contracts: scenario.contracts.clone(),
            pricing_policy: scenario.pricing_policy.clone(),
            demand_model: scenario.demand_model.clone(),
            segments: scenario.segments.clone(),
        }
    }
}
