use std::collections::BTreeMap;

use crate::demand::customer_segment::CustomerSegment;
use crate::demand::demand_engine::DemandEngine;
use crate::demand::demand_model::DemandModel;
use crate::events::event_log::EventLog;
use crate::finance::ledger::Ledger;
use crate::model::good::Good;
use crate::model::inventory::Inventory;
use crate::model::store_id::StoreId;
use crate::time::tick::Tick;

#[test]
fn demand_reduces_stock() {
    let mut inv = Inventory::new(StoreId(0));
    inv.add_stock(Good::Widget, 100);
    let mut inventories = BTreeMap::new();
    inventories.insert(StoreId(0), inv);
    let mut ledger = Ledger::new();
    let mut log = EventLog::new();

    let segments = vec![CustomerSegment {
        name: "default".into(),
        base_demand: 10,
        price_sensitivity: 50,
        good: Good::Widget,
    }];
    let model = DemandModel::default();

    DemandEngine::process_demand(
        &Tick(0),
        &model,
        &segments,
        42,
        &mut inventories,
        &mut ledger,
        &mut log,
    );
    assert!(inventories[&StoreId(0)].get_stock(&Good::Widget) < 100);
}
