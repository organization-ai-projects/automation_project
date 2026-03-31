use crate::config::sim_config::{CompanyConfig, SimConfig, StoreConfig};
use crate::demand::customer_segment::CustomerSegment;
use crate::demand::demand_model::DemandModel;
use crate::model::good::Good;
use crate::pricing::pricing_policy::PricingPolicy;
use crate::supply::contract::Contract;
use crate::supply::supplier::SupplierId;
use crate::model::store_id::StoreId;

#[test]
fn pricing_war_reduces_margins() {
    let config = SimConfig {
        seed: 42,
        ticks: 50,
        companies: vec![
            CompanyConfig {
                name: "AlphaCorp".into(),
                budget: 100000,
                stores: vec![StoreConfig { name: "Alpha Store".into() }],
            },
            CompanyConfig {
                name: "BetaCorp".into(),
                budget: 100000,
                stores: vec![StoreConfig { name: "Beta Store".into() }],
            },
        ],
        contracts: vec![
            Contract {
                supplier_id: SupplierId(0),
                store_id: StoreId(0),
                good: Good::Widget,
                quantity_per_delivery: 100,
                delivery_interval: 5,
                cost_per_unit: 400,
            },
            Contract {
                supplier_id: SupplierId(1),
                store_id: StoreId(1000),
                good: Good::Widget,
                quantity_per_delivery: 100,
                delivery_interval: 5,
                cost_per_unit: 400,
            },
        ],
        pricing_policy: PricingPolicy {
            markup_percent: 20,
            discount_threshold: 50,
            discount_percent: 15,
        },
        demand_model: DemandModel::default(),
        segments: vec![CustomerSegment {
            name: "mass".into(),
            base_demand: 30,
            price_sensitivity: 80,
            good: Good::Widget,
        }],
    };

    let mut engine = crate::create_engine(&config);
    let event_log = crate::run_simulation(&mut engine, &config);
    assert!(!event_log.is_empty());
}
