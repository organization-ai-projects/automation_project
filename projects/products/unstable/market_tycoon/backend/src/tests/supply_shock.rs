use crate::config::sim_config::{CompanyConfig, SimConfig, StoreConfig};
use crate::demand::customer_segment::CustomerSegment;
use crate::demand::demand_model::DemandModel;
use crate::model::good::Good;
use crate::pricing::pricing_policy::PricingPolicy;
use crate::report::sim_report::SimReport;

#[test]
fn no_supply_means_no_sales() {
    let config = SimConfig {
        seed: 99,
        ticks: 20,
        companies: vec![CompanyConfig {
            name: "NoSupply".into(),
            budget: 10000,
            stores: vec![StoreConfig {
                name: "Empty Store".into(),
            }],
        }],
        contracts: vec![],
        pricing_policy: PricingPolicy::default(),
        demand_model: DemandModel::default(),
        segments: vec![CustomerSegment {
            name: "eager".into(),
            base_demand: 100,
            price_sensitivity: 10,
            good: Good::Widget,
        }],
    };

    let mut engine = crate::create_engine(&config);
    let event_log = crate::run_simulation(&mut engine, &config);
    let report = SimReport::generate(&engine, &event_log, &config);
    assert_eq!(report.net_profit, 0);
}
