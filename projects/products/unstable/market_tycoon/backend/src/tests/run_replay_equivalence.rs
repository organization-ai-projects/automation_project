use crate::config::sim_config::{CompanyConfig, SimConfig, StoreConfig};
use crate::demand::customer_segment::CustomerSegment;
use crate::demand::demand_model::DemandModel;
use crate::model::good::Good;
use crate::model::store_id::StoreId;
use crate::pricing::pricing_policy::PricingPolicy;
use crate::report::sim_report::SimReport;
use crate::supply::contract::Contract;
use crate::supply::supplier::SupplierId;

#[test]
fn two_runs_same_seed_produce_same_hash() {
    let config = SimConfig {
        seed: 42,
        ticks: 50,
        companies: vec![CompanyConfig {
            name: "TestCo".into(),
            budget: 100000,
            stores: vec![StoreConfig {
                name: "Store A".into(),
            }],
        }],
        contracts: vec![Contract {
            supplier_id: SupplierId(0),
            store_id: StoreId(0),
            good: Good::Widget,
            quantity_per_delivery: 30,
            delivery_interval: 5,
            cost_per_unit: 300,
        }],
        pricing_policy: PricingPolicy::default(),
        demand_model: DemandModel::default(),
        segments: vec![CustomerSegment {
            name: "default".into(),
            base_demand: 15,
            price_sensitivity: 50,
            good: Good::Widget,
        }],
    };

    let mut engine1 = crate::create_engine(&config);
    let log1 = crate::run_simulation(&mut engine1, &config);
    let report1 = SimReport::generate(&engine1, &log1, &config);

    let mut engine2 = crate::create_engine(&config);
    let log2 = crate::run_simulation(&mut engine2, &config);
    let report2 = SimReport::generate(&engine2, &log2, &config);

    assert_eq!(report1.run_hash, report2.run_hash);
    assert_eq!(report1.event_count, report2.event_count);
    assert_eq!(report1.net_profit, report2.net_profit);
}
