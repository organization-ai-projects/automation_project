use crate::config::sim_config::{CompanyConfig, SimConfig, StoreConfig};
use crate::demand::customer_segment::CustomerSegment;
use crate::demand::demand_model::DemandModel;
use crate::model::good::Good;
use crate::pricing::pricing_policy::PricingPolicy;
use crate::scenario::scenario::Scenario;

#[test]
fn from_scenario_preserves_seed_and_ticks() {
    let scenario = Scenario {
        name: "test".into(),
        companies: vec![CompanyConfig {
            name: "Corp".into(),
            budget: 10000,
            stores: vec![StoreConfig { name: "Store A".into() }],
        }],
        contracts: vec![],
        pricing_policy: PricingPolicy::default(),
        demand_model: DemandModel::default(),
        segments: vec![CustomerSegment {
            name: "default".into(),
            base_demand: 10,
            price_sensitivity: 50,
            good: Good::Widget,
        }],
    };

    let config = SimConfig::from_scenario(&scenario, 99, 200);
    assert_eq!(config.seed, 99);
    assert_eq!(config.ticks, 200);
    assert_eq!(config.companies.len(), 1);
}
