use serde::{Deserialize, Serialize};

use crate::config::sim_config::{CompanyConfig, StoreConfig};
use crate::demand::customer_segment::CustomerSegment;
use crate::demand::demand_model::DemandModel;
use crate::pricing::pricing_policy::PricingPolicy;
use crate::supply::contract::Contract;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub companies: Vec<CompanyConfig>,
    pub contracts: Vec<Contract>,
    pub pricing_policy: PricingPolicy,
    pub demand_model: DemandModel,
    pub segments: Vec<CustomerSegment>,
}
