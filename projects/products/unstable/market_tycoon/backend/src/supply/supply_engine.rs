use std::collections::BTreeMap;

use crate::events::event_log::EventLog;
use crate::events::sim_event::SimEvent;
use crate::finance::ledger::Ledger;
use crate::finance::transaction::Transaction;
use crate::model::inventory::Inventory;
use crate::model::store_id::StoreId;
use crate::supply::contract::Contract;
use crate::time::tick::Tick;

pub struct SupplyEngine;

impl SupplyEngine {
    pub fn process_delivery(
        contract: &Contract,
        tick: &Tick,
        inventories: &mut BTreeMap<StoreId, Inventory>,
        ledger: &mut Ledger,
        event_log: &mut EventLog,
    ) {
        if contract.delivery_interval == 0 {
            return;
        }
        if tick.value() % contract.delivery_interval != 0 {
            return;
        }

        if let Some(inv) = inventories.get_mut(&contract.store_id) {
            inv.add_stock(contract.good, contract.quantity_per_delivery);
            let cost = contract.cost_per_unit * contract.quantity_per_delivery as i64;
            ledger.record(Transaction::supply_cost(*tick, contract.store_id, cost));
            event_log.push(SimEvent::delivery(
                *tick,
                contract.store_id,
                contract.good,
                contract.quantity_per_delivery,
            ));
        }
    }
}
