use std::collections::BTreeMap;

use crate::events::event_log::EventLog;
use crate::finance::ledger::Ledger;
use crate::model::good::Good;
use crate::model::inventory::Inventory;
use crate::model::store_id::StoreId;
use crate::supply::contract::Contract;
use crate::supply::supplier::SupplierId;
use crate::supply::supply_engine::SupplyEngine;
use crate::time::tick::Tick;

#[test]
fn delivery_on_interval() {
    let contract = Contract {
        supplier_id: SupplierId(1),
        store_id: StoreId(0),
        good: Good::Widget,
        quantity_per_delivery: 20,
        delivery_interval: 5,
        cost_per_unit: 300,
    };

    let mut inventories = BTreeMap::new();
    inventories.insert(StoreId(0), Inventory::new(StoreId(0)));
    let mut ledger = Ledger::new();
    let mut log = EventLog::new();

    SupplyEngine::process_delivery(&contract, &Tick(5), &mut inventories, &mut ledger, &mut log);
    assert_eq!(inventories[&StoreId(0)].get_stock(&Good::Widget), 20);
}

#[test]
fn no_delivery_off_interval() {
    let contract = Contract {
        supplier_id: SupplierId(1),
        store_id: StoreId(0),
        good: Good::Widget,
        quantity_per_delivery: 20,
        delivery_interval: 5,
        cost_per_unit: 300,
    };

    let mut inventories = BTreeMap::new();
    inventories.insert(StoreId(0), Inventory::new(StoreId(0)));
    let mut ledger = Ledger::new();
    let mut log = EventLog::new();

    SupplyEngine::process_delivery(&contract, &Tick(3), &mut inventories, &mut ledger, &mut log);
    assert_eq!(inventories[&StoreId(0)].get_stock(&Good::Widget), 0);
}
