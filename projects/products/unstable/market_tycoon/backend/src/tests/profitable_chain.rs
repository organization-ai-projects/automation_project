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
fn single_chain_is_profitable() {
    let config = SimConfig {
        seed: 1,
        ticks: 100,
        companies: vec![CompanyConfig {
            name: "ProfitCo".into(),
            budget: 50000,
            stores: vec![StoreConfig {
                name: "Main Store".into(),
            }],
        }],
        contracts: vec![Contract {
            supplier_id: SupplierId(0),
            store_id: StoreId(0),
            good: Good::Widget,
            quantity_per_delivery: 50,
            delivery_interval: 10,
            cost_per_unit: 300,
        }],
        pricing_policy: PricingPolicy {
            markup_percent: 50,
            discount_threshold: 200,
            discount_percent: 5,
        },
        demand_model: DemandModel::default(),
        segments: vec![CustomerSegment {
            name: "premium".into(),
            base_demand: 20,
            price_sensitivity: 30,
            good: Good::Widget,
        }],
    };

    let mut engine = crate::create_engine(&config);
    let event_log = crate::run_simulation(&mut engine, &config);
    let report = SimReport::generate(&engine, &event_log, &config);
    assert!(!report.run_hash.is_empty());
}
